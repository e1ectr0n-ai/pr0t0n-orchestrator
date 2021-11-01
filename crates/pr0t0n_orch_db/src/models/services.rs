use crate::{
    err::Error,
    models::enums::{HealthStatus, ServiceType, StringEnum},
    models::generic::*,
    schema::services::{self},
};
use std::convert::TryFrom;

/// Public representation of a service with typed enums.
#[derive(Debug, Default, Clone)]
pub struct Service {
    pub service_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub address: String,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
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
        })
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
            input_config_id: None,
            output_config_id: None,
            processor_config_id: None,
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

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::test_utils::temp_asset_group_test;
    use diesel::PgConnection;

    #[test]
    fn create_input_service() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let mut service = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_service",
                address: "localhost:2222",
                service_type: ServiceType::Input,
                health_status: HealthStatus::Healthy,
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", service);

            println!("Made an update");
            service.health_status = HealthStatus::Disconnected;
            service.update(conn)?;

            let read_service = Service::find(conn, service.service_id)?;
            println!("Queried {:#?}", read_service);
            Ok(())
        })
        .unwrap();
    }
}
