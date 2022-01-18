use std::collections::HashMap;
use std::convert::TryFrom;

use crate::models::{assets::*, generic::*};
use crate::schema::configs;
use crate::Error;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Debug)]
#[primary_key(config_id, name)]
pub struct Config {
    pub config_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub description: String,
    pub json_config: String,
}
impl Config {
    pub fn get_names(
        conn: &PgConnection,
        config_ids: &[i32],
    ) -> Result<HashMap<i32, String>, Error> {
        let mut results: Vec<(i32, String)> = configs::dsl::configs
            .filter(configs::config_id.eq_any(config_ids))
            .select((configs::config_id, configs::name))
            .get_results(conn)?;

        let mut map = HashMap::new();
        for (config_id, config_name) in results.drain(..) {
            map.insert(config_id, config_name);
        }
        Ok(map)
    }

    pub fn get_ids(
        conn: &PgConnection,
        asset_group_id: i32,
    ) -> Result<HashMap<String, i32>, Error> {
        let mut results: Vec<(i32, String)> = configs::dsl::configs
            .filter(configs::asset_group_id.eq(asset_group_id))
            .select((configs::config_id, configs::name))
            .get_results(conn)?;

        let mut map = HashMap::new();
        for (config_id, config_name) in results.drain(..) {
            map.insert(config_name, config_id);
        }
        Ok(map)
    }

    pub fn delete_all(conn: &PgConnection, config_ids: &[i32]) -> Result<usize, Error> {
        let num_deleted =
            diesel::delete(configs::dsl::configs.filter(configs::config_id.eq_any(config_ids)))
                .execute(conn)?;
        Ok(num_deleted)
    }

    /// Custom update for service.
    pub fn update(&self, conn: &PgConnection) -> Result<usize, Error> {
        let result: usize = diesel::update(configs::table)
            .set((
                configs::asset_group_id.eq(self.asset_group_id),
                configs::description.eq(self.description.clone()),
                configs::json_config.eq(self.json_config.clone()),
            ))
            .execute(conn)?;
        Ok(result)
    }
}
impl Asset for Config {
    /// Get all services for an asset_group_id.
    fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let results: Vec<Self> = configs::table
            .filter(configs::asset_group_id.eq(asset_group_id))
            .get_results(conn)?;
        Ok(results)
    }

    fn get_string_id(&self) -> &str {
        &self.name
    }
}
impl DbDelete for Config {
    type Table = configs::table;
}
impl DbFind for Config {
    type Table = configs::table;
}
impl DbUpdate for Config {
    type Table = configs::table;
}

#[derive(Insertable, Debug, Default)]
#[table_name = "configs"]
pub struct NewConfig<'a> {
    pub asset_group_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub json_config: &'a str,
}
impl DbInsert for NewConfig<'_> {
    type Table = configs::table;
    type Return = Config;
}
impl DbInsertAll for Vec<NewConfig<'_>> {
    type Table = configs::table;
    type Return = Config;
}

/// Config representation for ergonomic config.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct ConfigRepr {
    pub name: String,
    pub description: String,
    pub json_config: serde_json::Value,

    #[serde(skip)]
    pub json_config_str: String,
}
impl ConfigRepr {
    fn as_insertable(&mut self, asset_group_id: i32) -> NewConfig {
        self.json_config_str = self.json_config.to_string();
        NewConfig {
            asset_group_id,
            name: &self.name,
            description: &self.description,
            json_config: &self.json_config_str,
        }
    }
}
impl TryFrom<Config> for ConfigRepr {
    type Error = Error;
    fn try_from(config: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            name: config.name,
            description: config.description,
            json_config: serde_json::from_str(&config.json_config)?,
            json_config_str: "".to_string(),
        })
    }
}
impl<'a> AssetRepr<'a> for ConfigRepr {
    type Asset = Config;

    fn try_merge_asset(&self, asset: &mut Self::Asset) -> Result<(), Error> {
        asset.name = self.name.clone();
        asset.description = self.name.clone();
        asset.json_config = serde_json::to_string(&self.json_config)?;
        Ok(())
    }

    /// Gets the identifier string for this item.
    fn get_string_id(&self) -> &str {
        &self.name
    }

    /// Given a list of representations, syncs the tables in the database with this list.
    /// This sync includes inserting new entries, updating existing entries, and deleting removed
    /// entries.
    fn sync_db(
        conn: &PgConnection,
        asset_group_id: i32,
        reprs: &mut Vec<Self>,
    ) -> Result<(), Error> {
        let existing = Config::get_group_map(conn, asset_group_id)?;

        let (mut to_insert, to_update, to_delete) = Self::partition_diff(existing, reprs)?;

        let new_configs: Vec<NewConfig> = to_insert
            .iter_mut()
            .map(|asset| asset.as_insertable(asset_group_id))
            .collect();
        println!("Insert configs: {:#?}", new_configs);
        new_configs.insert_all(conn)?;
        println!("Inserted all configs.");

        for (_, config) in to_update {
            println!("Updating config {:#?}", config);
            config.update(conn)?;
        }
        println!("Updated all configs.");

        let delete_ids: Vec<i32> = to_delete.iter().map(|asset| asset.config_id).collect();
        Config::delete_all(conn, &delete_ids)?;

        Ok(())
    }

    fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let configs = Config::get_group(conn, asset_group_id)?;
        let mut reprs: Vec<ConfigRepr> = Vec::with_capacity(configs.len());
        for config in configs {
            reprs.push(ConfigRepr::try_from(config)?);
        }
        Ok(reprs)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::testing::temp_asset_group_test;
    use diesel::PgConnection;

    #[test]
    fn create_config() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let input = NewConfig {
                asset_group_id: asset_group.asset_group_id,
                name: "test_input_config",
                description: "Test config.",
                json_config: "{}",
            }
            .insert(conn)?;
            println!("Inserted {:#?}", input);
            Ok(())
        })
        .unwrap();
    }
}
