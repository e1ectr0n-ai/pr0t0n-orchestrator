use crate::{
    err::Error,
    schema::input_configs::{self},
};
use diesel::{pg::Pg, PgConnection, RunQueryDsl};

#[derive(Queryable, Debug)]
pub struct InputConfig {
    pub input_config_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub description: String,
    pub max_sample_rate: f32,
    pub json_config: Option<String>,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "input_configs"]
pub struct NewInputConfig<'a> {
    pub asset_group_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub max_sample_rate: f32,
    pub json_config: Option<&'a str>,
}
impl<'a> NewInputConfig<'a> {
    pub fn insert(&'a self, conn: &PgConnection) -> Result<InputConfig, Error> {
        let query = diesel::insert_into(input_configs::table).values(self);
        if cfg!(debug_assertions) {
            println!("{}", diesel::debug_query::<Pg, _>(&query));
        }
        query.get_result(conn).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::test_utils::temp_asset_group_test;
    use diesel::PgConnection;

    #[test]
    fn create_input_service() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let input = NewInputConfig {
                asset_group_id: asset_group.asset_group_id,
                name: "test_input_config",
                description: "Test config.",
                max_sample_rate: 10.,
                json_config: Some("Test"),
            }
            .insert(conn)?;
            println!("Inserted {:#?}", input);
            Ok(())
        })
        .unwrap();
    }
}
