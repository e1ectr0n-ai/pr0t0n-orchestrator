use std::collections::{BTreeMap, HashMap};

use crate::{
    errors::Error,
    models::{
        configs::Config,
        enums::{HealthStatus, ServiceType},
        generic::{DbDelete, DbFind, DbInsert, DbInsertAll, DbUpdate},
        service_edges::ServiceEdge,
    },
    schema::{service_edges, services},
};
use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::assets::{Asset, AssetRepr};

/// Public representation of a service with typed enums.
#[derive(Queryable, AsChangeset, Debug, Default, Clone)]
#[primary_key(service_id)]
pub struct Service {
    pub service_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub address: String,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
    pub config_id: Option<i32>,
}
impl Service {
    pub fn get_addr_to_id(
        conn: &PgConnection,
        asset_group_id: i32,
    ) -> Result<HashMap<String, i32>, Error> {
        let mut results: Vec<(i32, String)> = services::dsl::services
            .filter(services::asset_group_id.eq(asset_group_id))
            .select((services::service_id, services::address))
            .get_results(conn)?;

        let mut map = HashMap::new();
        for (service_id, address) in results.drain(..) {
            map.insert(address, service_id);
        }
        Ok(map)
    }

    /// Get all services for an asset_group_id.
    pub fn find_by_addr(conn: &PgConnection, address: &str) -> Result<Self, Error> {
        let result: Service = services::table
            .filter(services::address.eq(address))
            .get_result(conn)?;
        Ok(result)
    }

    /// Get all services for an asset_group_id.
    pub fn find_by_addrs(conn: &PgConnection, addresses: &[&str]) -> Result<Vec<Self>, Error> {
        let results: Vec<Service> = services::table
            .filter(services::address.eq_any(addresses))
            .get_results(conn)?;
        Ok(results)
    }

    /// Get all services for an asset_group_id.
    pub fn get_graph(
        conn: &PgConnection,
        asset_group_id: i32,
    ) -> Result<(Vec<Self>, Vec<ServiceEdge>), Error> {
        let services = Self::get_group(conn, asset_group_id)?;
        let edges = ServiceEdge::get_group(conn, asset_group_id)?;
        Ok((services, edges))
    }

    /// Given an address, if a service with that address already exists, create it.
    /// Otherwise update it's connection status to healthy.
    pub fn upsert_healthy_address(
        conn: &PgConnection,
        asset_group_id: i32,
        address: &str,
    ) -> Result<(), Error> {
        let new_service = NewService {
            asset_group_id,
            name: address,
            address,
            health_status: HealthStatus::Healthy,
            service_type: ServiceType::Input,
            ..Default::default()
        };
        diesel::insert_into(services::table)
            .values(new_service)
            .on_conflict(services::address)
            .do_update()
            .set(services::health_status.eq(HealthStatus::Healthy))
            .execute(conn)?;
        Ok(())
    }

    /// Given an address, if a service with that address already exists, create it.
    /// Otherwise update it's connection status to healthy.
    pub fn disconnect_address(conn: &PgConnection, address: &str) -> Result<(), Error> {
        diesel::update(services::table.filter(services::address.eq(address)))
            .set(services::health_status.eq(HealthStatus::Disconnected)) // HealthStatus::Disconnected.as_str()))
            .execute(conn)?;
        Ok(())
    }

    /// Get all output services for a given service.
    pub fn get_outputs(&self, conn: &PgConnection) -> Result<Vec<Self>, Error> {
        let results: Vec<(ServiceEdge, Service)> = service_edges::table
            .filter(service_edges::input_service_id.eq(self.service_id))
            .inner_join(
                services::table.on(services::service_id.eq(service_edges::output_service_id)),
            )
            .get_results(conn)?;
        Ok(results
            .into_iter()
            .map(|(_edge, service)| service)
            .collect())
    }

    /// Get all input services for a given service.
    pub fn get_inputs(&self, conn: &PgConnection) -> Result<Vec<Self>, Error> {
        let results: Vec<(ServiceEdge, Service)> = service_edges::table
            .filter(service_edges::output_service_id.eq(self.service_id))
            .inner_join(
                services::table.on(services::service_id.eq(service_edges::input_service_id)),
            )
            .get_results(conn)?;
        Ok(results
            .into_iter()
            .map(|(_edge, service)| service)
            .collect())
    }

    pub fn update_outputs(
        &self,
        conn: &PgConnection,
        asset_group_id: i32,
        addr_to_id: &HashMap<String, i32>,
        repr: &ServiceRepr,
    ) -> Result<(), Error> {
        // Update service edges.
        let input_service_id = self.service_id;
        let mut new_edges: Vec<ServiceEdge> = Vec::with_capacity(repr.output_addresses.len());
        for output_addr in &repr.output_addresses {
            if let Some(&output_service_id) = addr_to_id.get(output_addr) {
                new_edges.push(ServiceEdge {
                    asset_group_id,
                    input_service_id,
                    output_service_id,
                });
            } else {
                return Err(Error::DatabaseSyncError(format!(
                    "Failed to find service with address '{}'",
                    output_addr
                )));
            }
        }
        ServiceEdge::delete_outputs(conn, input_service_id)?;
        new_edges.insert_all(conn)?;
        Ok(())
    }
}
impl DbDelete for Service {
    type Table = services::table;
}
impl DbFind for Service {
    type Table = services::table;
}
impl DbUpdate for Service {
    type Table = services::table;
}
impl Asset for Service {
    /// Get all services for an asset_group_id.
    fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let results: Vec<Service> = services::table
            .filter(services::asset_group_id.eq(asset_group_id))
            .get_results(conn)?;
        Ok(results)
    }

    fn get_string_id(&self) -> &str {
        &self.address
    }
}

/// Public representation of a service with typed enums.
#[derive(Insertable, Debug, Default, Clone)]
#[table_name = "services"]
pub struct NewService<'a> {
    pub asset_group_id: i32,
    pub name: &'a str,
    pub address: &'a str,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
    pub config_id: Option<i32>,
}
impl DbInsert for NewService<'_> {
    type Table = services::table;
    type Return = Service;
}
impl DbInsertAll for Vec<NewService<'_>> {
    type Table = services::table;
    type Return = Service;
}

/// Service representation for ergonomic config.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct ServiceRepr {
    pub address: String,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
    pub name: String,
    pub output_addresses: Vec<String>,
    pub config_name: Option<String>,

    /// Populated automatically based on `config_name`
    #[serde(skip)]
    pub config_id: Option<i32>,
}
impl From<Service> for ServiceRepr {
    fn from(service: Service) -> Self {
        Self {
            address: service.address,
            service_type: service.service_type,
            health_status: service.health_status,
            name: service.name,
            config_id: service.config_id,
            ..Default::default()
        }
    }
}
impl ServiceRepr {
    fn as_insertable(&self, asset_group_id: i32) -> NewService {
        NewService {
            asset_group_id,
            name: &self.name,
            address: &self.address,
            service_type: self.service_type,
            health_status: self.health_status,
            ..Default::default()
        }
    }
}
impl<'a> AssetRepr<'a> for ServiceRepr {
    type Asset = Service;

    /// Get all services with associated outputs addresses for an asset_group_id.
    fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let (mut services, edges) = Service::get_graph(conn, asset_group_id)?;

        let mut config_ids: Vec<i32> = Vec::new();
        let mut service_map: BTreeMap<i32, Self> = BTreeMap::new();
        for service in services.drain(..) {
            if let Some(config_id) = service.config_id {
                config_ids.push(config_id);
            }
            service_map.insert(service.service_id, Self::from(service));
        }

        // Populate config names.
        let config_names: HashMap<i32, String> = Config::get_names(conn, &config_ids)?;
        // println!("Got config names")
        for repr in service_map.values_mut() {
            if let Some(config_id) = repr.config_id {
                if let Some(config_name) = config_names.get(&config_id) {
                    repr.config_name = Some(config_name.clone());
                }
            }
        }

        // One pass over edges to attach addresses from outputs to the input services' list.
        for edge in edges {
            let output_address =
                if let Some(output_service) = service_map.get(&edge.output_service_id) {
                    output_service.address.clone()
                } else {
                    continue;
                };
            println!("output addr: {:?}", output_address);
            if let Some(input_service) = service_map.get_mut(&edge.input_service_id) {
                input_service.output_addresses.push(output_address);
            }
        }

        // Lastly, drain back into a vector, sorted by key (service_id)
        Ok(service_map.into_values().collect())
    }

    fn try_merge_asset(&self, asset: &mut Self::Asset) -> Result<(), Error> {
        asset.name = self.name.clone();
        asset.address = self.address.clone();
        Ok(())
    }

    /// Gets the identifier string for this item.
    fn get_string_id(&self) -> &str {
        &self.name
    }

    fn sync_db(
        conn: &PgConnection,
        asset_group_id: i32,
        reprs: &mut Vec<Self>,
    ) -> Result<(), Error> {
        let existing = Self::Asset::get_group_map(conn, asset_group_id)?;
        let (to_insert, to_update, to_delete) = Self::partition_diff(existing, reprs)?;

        // Get config ids for names.
        let config_ids: HashMap<String, i32> = Config::get_ids(conn, asset_group_id)?;
        let mut new_services: Vec<NewService> = Vec::with_capacity(to_insert.len());
        for repr in &to_insert {
            let mut new_service = repr.as_insertable(asset_group_id);
            if let Some(config_name) = &repr.config_name {
                if let Some(&config_id) = config_ids.get(config_name) {
                    new_service.config_id = Some(config_id);
                } else {
                    return Err(Error::DatabaseSyncError(format!(
                        "Failed to find config '{}'",
                        config_name
                    )));
                }
            }
            new_services.push(new_service);
        }

        // Insert new services.

        println!("Inserting new services: {:#?}", new_services);
        let inserted_services: Vec<Service> = new_services.insert_all(conn)?;

        // New service modifications are completed, so collect the address to ID map here.
        let addr_to_id: HashMap<String, i32> = Service::get_addr_to_id(conn, asset_group_id)?;

        // Connect new services to their outputs.
        for (repr, service) in to_insert.iter().zip(&inserted_services) {
            service.update_outputs(conn, asset_group_id, &addr_to_id, &repr)?;
        }

        // Update existing services.
        for (repr, service) in &to_update {
            println!("Updating service: {:#?}", service);
            service.update_outputs(conn, asset_group_id, &addr_to_id, &repr)?;
            service.update(conn)?;
        }

        let delete_ids: Vec<i32> = to_delete.iter().map(|asset| asset.service_id).collect();
        Config::delete_all(conn, &delete_ids)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::testing::temp_asset_group_test;
    use diesel::PgConnection;

    pub fn setup_edges(service_pairs: Vec<(&Service, &Service)>) -> Vec<ServiceEdge> {
        service_pairs
            .iter()
            .map(|(input, output)| ServiceEdge {
                asset_group_id: input.asset_group_id,
                input_service_id: input.service_id,
                output_service_id: output.service_id,
            })
            .collect()
    }

    #[test]
    fn test_services() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let config = NewConfig {
                asset_group_id: asset_group.asset_group_id,
                name: "Config1",
                description: "Test config",
                json_config: "{}",
            }
            .insert(conn)?;

            let mut input_service = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_input1",
                address: "test_input1:2222",
                service_type: ServiceType::Input,
                health_status: HealthStatus::Healthy,
                config_id: Some(config.config_id),
                ..Default::default()
            }
            .insert(conn)?;

            input_service.health_status = HealthStatus::Disconnected;
            input_service.update(conn)?;

            let read_service = Service::find(conn, input_service.service_id)?;
            assert_eq!(read_service.health_status, HealthStatus::Disconnected);

            let processor_service1 = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_processor1",
                address: "test_processor1:2222",
                service_type: ServiceType::Processor,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;

            let processor_service2 = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_processor2",
                address: "test_processor2:2222",
                service_type: ServiceType::Processor,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;

            let output_service = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_output1",
                address: "test_output1:2222",
                service_type: ServiceType::Output,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;

            // Setup edges in the service graph.
            let _edges = setup_edges(vec![
                (&input_service, &processor_service1),
                (&input_service, &processor_service2),
                (&processor_service1, &output_service),
                (&processor_service2, &output_service),
            ])
            .insert_all(conn)?;

            // Check the output graph.
            let input_service_outputs = input_service.get_outputs(conn)?;
            assert_eq!(input_service_outputs.len(), 2);

            let processor1_outputs = processor_service1.get_outputs(conn)?;
            assert_eq!(processor1_outputs.len(), 1);

            let processor2_outputs = processor_service1.get_outputs(conn)?;
            assert_eq!(processor2_outputs.len(), 1);

            let output_service_inputs = output_service.get_inputs(conn)?;
            assert_eq!(output_service_inputs.len(), 2);

            let services_with_outputs = ServiceRepr::get_group(conn, asset_group.asset_group_id)?;
            println!(
                "Services with outputs:\n {}",
                serde_json::to_string_pretty(&services_with_outputs)?
            );

            Service::delete(conn, processor_service2.service_id)?;
            drop(processor_service2);

            // Check the output graph after the deletion.
            let input_service_outputs = input_service.get_outputs(conn)?;
            assert_eq!(input_service_outputs.len(), 1);

            let processor1_outputs = processor_service1.get_outputs(conn)?;
            assert_eq!(processor1_outputs.len(), 1);

            let output_service_inputs = output_service.get_inputs(conn)?;
            assert_eq!(output_service_inputs.len(), 1);

            let services_with_outputs = ServiceRepr::get_group(conn, asset_group.asset_group_id);
            println!("Services with outputs {:#?}", services_with_outputs);

            // Check upsetting new connection.
            Service::disconnect_address(conn, &output_service.address)?;
            println!("Disconnected address.");
            Service::upsert_healthy_address(
                conn,
                output_service.asset_group_id,
                &output_service.address,
            )?;
            println!("Updated healthy address.");

            let new_addr = "new_address1:123";
            Service::upsert_healthy_address(conn, output_service.asset_group_id, new_addr)?;
            let new_service = Service::find_by_addr(conn, new_addr);
            println!("Got new service: {:#?}", new_service);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_connect_service() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let mut input_service = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_input2",
                address: "test_input2:2222",
                service_type: ServiceType::Input,
                health_status: HealthStatus::Disconnected,
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", input_service);

            // Check updating for existing service.
            Service::upsert_healthy_address(
                conn,
                input_service.asset_group_id,
                &input_service.address,
            )?;
            input_service = Service::find(conn, input_service.service_id)?;
            assert_eq!(input_service.health_status, HealthStatus::Healthy);
            Service::disconnect_address(conn, &input_service.address)?;
            input_service = Service::find(conn, input_service.service_id)?;
            assert_eq!(input_service.health_status, HealthStatus::Disconnected);

            let new_addr = "new_address2:123";
            Service::upsert_healthy_address(conn, input_service.asset_group_id, new_addr)?;
            let mut new_service = Service::find_by_addr(conn, new_addr)?;
            assert_eq!(new_service.health_status, HealthStatus::Healthy);
            Service::disconnect_address(conn, new_addr)?;
            new_service = Service::find_by_addr(conn, new_addr)?;
            assert_eq!(new_service.health_status, HealthStatus::Disconnected);
            Ok(())
        })
        .unwrap();
    }
}
