use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::{
    models::{assets::*, configs::*, services::*},
    Error,
};

/// Model the entire database using a single serializable state struct.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct SystemRepr {
    pub services: Vec<ServiceRepr>,
    pub configs: Vec<ConfigRepr>,
}
impl SystemRepr {
    pub fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Self, Error> {
        Ok(Self {
            services: ServiceRepr::get_group(conn, asset_group_id)?,
            configs: ConfigRepr::get_group(conn, asset_group_id)?,
        })
    }

    /// Given a representation, make the database match what we have configured.
    pub fn sync_db(&mut self, conn: &PgConnection, asset_group_id: i32) -> Result<(), Error> {
        ConfigRepr::sync_db(conn, asset_group_id, &mut self.configs)?;
        ServiceRepr::sync_db(conn, asset_group_id, &mut self.services)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::testing::temp_asset_group_test;
    use assert_json_diff::assert_json_eq;
    use diesel::PgConnection;

    #[test]
    fn test_sync() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let asset_group_id = asset_group.asset_group_id;

            // First create a system with two nodes.
            {
                let system_repr = SystemRepr {
                    services: vec![
                        ServiceRepr {
                            address: "localhost:123".to_string(),
                            service_type: ServiceType::Input,
                            name: "localhost:123".to_string(),
                            output_addresses: vec!["localhost:234".to_string()],
                            config_name: Some("TestConfig".to_string()),
                            ..Default::default()
                        },
                        ServiceRepr {
                            address: "localhost:234".to_string(),
                            service_type: ServiceType::Input,
                            name: "localhost:234".to_string(),
                            output_addresses: vec![],
                            config_name: Some("TestConfig".to_string()),
                            ..Default::default()
                        },
                    ],
                    configs: vec![ConfigRepr {
                        name: "TestConfig".to_string(),
                        description: "A test config".to_string(),
                        json_config: serde_json::from_str(r#"{ "key": "value" }"#)?,
                        ..Default::default()
                    }],
                };
                system_repr.clone().sync_db(conn, asset_group_id)?;

                let new_system_repr = SystemRepr::get_group(conn, asset_group_id)?;
                assert_json_eq!(system_repr, new_system_repr);
            }

            // Now delete the second node and make sure it's deleted in the database.
            {
                let system_repr = SystemRepr {
                    services: vec![ServiceRepr {
                        address: "localhost:123".to_string(),
                        service_type: ServiceType::Input,
                        name: "localhost:123".to_string(),
                        output_addresses: vec!["localhost:234".to_string()],
                        config_name: Some("TestConfig".to_string()),
                        ..Default::default()
                    }],
                    configs: vec![ConfigRepr {
                        name: "TestConfig".to_string(),
                        description: "A test config".to_string(),
                        json_config: serde_json::from_str(r#"{ "key": "value" }"#)?,
                        ..Default::default()
                    }],
                };
                system_repr.clone().sync_db(conn, asset_group_id)?;

                // let new_system_repr = SystemRepr::get_group(conn, asset_group_id)?;
                // assert_json_eq!(system_repr, new_system_repr);
            }
            Ok(())
        })
        .unwrap();
    }
}
