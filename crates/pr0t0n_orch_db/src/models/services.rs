use crate::{
    errors::Error,
    models::enums::{HealthStatus, ServiceConfigId, ServiceType, StringEnum},
    models::generic::*,
    schema::{service_edges, services},
};
use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl};

use std::convert::{TryFrom, TryInto};

/// Public representation of a service with typed enums.
#[derive(Debug, Default, Clone)]
pub struct Service {
    pub service_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub address: String,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
    pub config_id: ServiceConfigId,
}
impl DbDelete for Service {
    type Table = services::table;
}
impl DbMappedFind for Service {
    type Table = services::table;
    type Queryable = QueryableService;
}
impl<'a> DbMappedUpdate<'a> for Service {
    type Table = services::table;
    type Insertable = InsertableService<'a>;
}
impl<'a> TryFrom<QueryableService> for Service {
    type Error = Error;
    fn try_from(queried: QueryableService) -> Result<Self, Self::Error> {
        Ok(Self {
            service_id: queried.service_id,
            asset_group_id: queried.asset_group_id,
            name: queried.name,
            address: queried.address,
            service_type: ServiceType::from_str(&queried.service_type)?,
            health_status: HealthStatus::from_str(&queried.health_status)?,
            config_id: ServiceConfigId::None,
        })
    }
}
impl Service {
    /// Get all services for an asset_group_id.
    pub fn get_all(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let results: Vec<QueryableService> = services::table
            .filter(services::asset_group_id.eq(asset_group_id))
            .get_results(conn)?;
        results
            .into_iter()
            .map(|service| service.try_into())
            .collect()
    }

    /// Get all services for an asset_group_id.
    pub fn find_by_addr(conn: &PgConnection, address: &str) -> Result<Self, Error> {
        let result: QueryableService = services::table
            .filter(services::address.eq(address))
            .get_result(conn)?;
        result.try_into()
    }

    /// Get all services for an asset_group_id.
    pub fn get_graph(
        conn: &PgConnection,
        asset_group_id: i32,
    ) -> Result<(Vec<Self>, Vec<ServiceEdge>), Error> {
        let services = Self::get_all(conn, asset_group_id)?;
        let edges = ServiceEdge::get_all(conn, asset_group_id)?;
        Ok((services, edges))
    }

    /// Given an address, if a service with that address already exists, create it.
    /// Otherwise update it's connection status to healthy.
    pub fn upsert_healthy_address(
        conn: &PgConnection,
        asset_group_id: i32,
        address: &str,
    ) -> Result<(), Error> {
        let new_service = InsertableService {
            asset_group_id,
            name: address,
            address,
            health_status: HealthStatus::Healthy.as_str(),
            service_type: ServiceType::Input.as_str(),
            ..Default::default()
        };
        diesel::insert_into(services::table)
            .values(new_service)
            .on_conflict(services::address)
            .do_update()
            .set(services::health_status.eq(HealthStatus::Healthy.as_str()))
            .execute(conn)?;
        Ok(())
    }

    /// Given an address, if a service with that address already exists, create it.
    /// Otherwise update it's connection status to healthy.
    pub fn disconnect_address(conn: &PgConnection, address: &str) -> Result<(), Error> {
        diesel::update(services::table.filter(services::address.eq(address)))
            .set(services::health_status.eq(HealthStatus::Disconnected.as_str()))
            .execute(conn)?;
        Ok(())
    }

    /// Get all output services for a given service.
    pub fn get_outputs(&self, conn: &PgConnection) -> Result<Vec<Self>, Error> {
        let results: Vec<(ServiceEdge, QueryableService)> = service_edges::table
            .filter(service_edges::input_service_id.eq(self.service_id))
            .inner_join(
                services::table.on(services::service_id.eq(service_edges::output_service_id)),
            )
            .get_results(conn)?;
        results
            .into_iter()
            .map(|(_edge, service)| service.try_into())
            .collect()
    }

    /// Get all input services for a given service.
    pub fn get_inputs(&self, conn: &PgConnection) -> Result<Vec<Self>, Error> {
        let results: Vec<(ServiceEdge, QueryableService)> = service_edges::table
            .filter(service_edges::output_service_id.eq(self.service_id))
            .inner_join(
                services::table.on(services::service_id.eq(service_edges::input_service_id)),
            )
            .get_results(conn)?;
        let iter = results.into_iter();
        iter.map(|(_edge, service)| service.try_into()).collect()
    }
}

/// Internal queryable Service.
#[derive(Queryable, Debug)]
pub struct QueryableService {
    pub service_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub address: String,
    pub service_type: String,
    pub health_status: String,
    pub input_config_id: Option<i32>,
    pub output_config_id: Option<i32>,
    pub processor_config_id: Option<i32>,
}

/// New service inserter arguments struct.
#[derive(Insertable, AsChangeset, Debug, Default)]
#[table_name = "services"]
pub struct InsertableService<'a> {
    pub asset_group_id: i32,
    pub name: &'a str,
    pub address: &'a str,
    pub service_type: &'a str,
    pub health_status: &'a str,
    pub input_config_id: Option<i32>,
    pub output_config_id: Option<i32>,
    pub processor_config_id: Option<i32>,
}
impl<'a> TryFrom<&'a NewService<'_>> for InsertableService<'a> {
    type Error = Error;
    fn try_from(new: &'a NewService<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_group_id: new.asset_group_id,
            name: new.name,
            address: new.address,
            service_type: new.service_type.as_str(),
            health_status: new.health_status.as_str(),
            input_config_id: None,
            output_config_id: None,
            processor_config_id: None,
        })
    }
}
impl<'a> TryFrom<&'a Service> for InsertableService<'a> {
    type Error = Error;
    fn try_from(service: &'a Service) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_group_id: service.asset_group_id,
            name: &service.name,
            address: &service.address,
            service_type: service.service_type.as_str(),
            health_status: service.health_status.as_str(),
            // TODO: set these using enum.
            input_config_id: service.config_id.input_config_id(),
            output_config_id: service.config_id.output_config_id(),
            processor_config_id: service.config_id.processor_config_id(),
        })
    }
}

/// Public representation of a service with typed enums.
#[derive(Debug, Default, Clone)]
pub struct NewService<'a> {
    pub asset_group_id: i32,
    pub name: &'a str,
    pub address: &'a str,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
}
impl<'a> DbMappedInsert<'a> for NewService<'a> {
    type Table = services::table;
    type Insertable = InsertableService<'a>;
    type Return = QueryableService;
    type MappedReturn = Service;
}

#[derive(Insertable, Queryable, Debug, Default)]
pub struct ServiceEdge {
    pub input_service_id: i32,
    pub output_service_id: i32,
    pub asset_group_id: i32,
}
impl DbFind for ServiceEdge {
    type Table = service_edges::table;
}
impl DbDelete for ServiceEdge {
    type Table = service_edges::table;
}
impl DbInsert for ServiceEdge {
    type Table = service_edges::table;
    type Return = Self;
}
impl DbInsertAll for Vec<ServiceEdge> {
    type Table = service_edges::table;
    type Return = ServiceEdge;
}
impl ServiceEdge {
    /// Get all services for an asset_group_id.
    pub fn get_all(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let results: Vec<Self> = service_edges::table
            .filter(service_edges::asset_group_id.eq(asset_group_id))
            .get_results(conn)?;
        Ok(results)
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
            let mut input_service = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_input1",
                address: "test_input1:2222",
                service_type: ServiceType::Input,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", input_service);

            input_service.health_status = HealthStatus::Disconnected;
            input_service.update(conn)?;

            let read_service = Service::find(conn, input_service.service_id)?;
            assert_eq!(read_service.health_status, HealthStatus::Disconnected);
            println!("Queried {:#?}", read_service);

            let processor_service1 = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_processor1",
                address: "test_processor1:2222",
                service_type: ServiceType::Processor,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", processor_service1);

            let processor_service2 = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_processor2",
                address: "test_processor2:2222",
                service_type: ServiceType::Processor,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", processor_service1);

            let output_service = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_output1",
                address: "test_output1:2222",
                service_type: ServiceType::Output,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", output_service);

            // Setup edges in the service graph.
            let edges = setup_edges(vec![
                (&input_service, &processor_service1),
                (&input_service, &processor_service2),
                (&processor_service1, &output_service),
                (&processor_service2, &output_service),
            ])
            .insert(conn)?;
            println!("Inserted {:#?}", edges);

            // Check the output graph.
            let input_service_outputs = input_service.get_outputs(conn)?;
            println!("Input service outputs {:#?}", input_service_outputs);
            assert_eq!(input_service_outputs.len(), 2);

            let processor1_outputs = processor_service1.get_outputs(conn)?;
            println!("Processor1 outputs {:#?}", processor1_outputs);
            assert_eq!(processor1_outputs.len(), 1);

            let processor2_outputs = processor_service1.get_outputs(conn)?;
            println!("Processor2 outputs {:#?}", processor1_outputs);
            assert_eq!(processor2_outputs.len(), 1);

            let output_service_inputs = output_service.get_inputs(conn)?;
            println!("Output service inputs {:#?}", output_service_inputs);
            assert_eq!(output_service_inputs.len(), 2);

            Service::delete(conn, processor_service2.service_id)?;
            drop(processor_service2);

            // Check the output graph after the deletion.
            let input_service_outputs = input_service.get_outputs(conn)?;
            println!("Input service outputs {:#?}", input_service_outputs);
            assert_eq!(input_service_outputs.len(), 1);

            let processor1_outputs = processor_service1.get_outputs(conn)?;
            println!("Processor1 outputs {:#?}", processor1_outputs);
            assert_eq!(processor1_outputs.len(), 1);

            let output_service_inputs = output_service.get_inputs(conn)?;
            println!("Output service inputs {:#?}", output_service_inputs);
            assert_eq!(output_service_inputs.len(), 1);

            // Check upsetting new connection.
            Service::disconnect_address(conn, &output_service.address)?;
            Service::upsert_healthy_address(
                conn,
                output_service.asset_group_id,
                &output_service.address,
            )?;

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
