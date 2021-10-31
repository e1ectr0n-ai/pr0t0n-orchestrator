use std::convert::TryFrom;

use crate::{
    err::Error,
    models::enums::{HealthStatus, ServiceType},
    schema::services::{self, dsl},
};
use diesel::{pg::Pg, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use super::enums::StringEnum;

/// Public representation of a service with typed enums.
#[derive(Debug, Default)]
pub struct Service {
    pub service_id: i32,
    pub asset_group_id: i32,
    pub name: String,
    pub address: String,
    pub service_type: ServiceType,
    pub health_status: HealthStatus,
}
impl Service {
    pub fn select(service_id: i32, conn: &PgConnection) -> Result<Service, Error> {
        let result: Result<Service_, Error> = services::table
            .find(service_id)
            .first::<Service_>(conn)
            .map_err(|e| e.into());
        Service::try_from(result?)
    }
    pub fn delete(id: i32, conn: &PgConnection) -> Result<usize, Error> {
        Service_::delete(id, conn)
    }
}
impl TryFrom<Service_> for Service {
    type Error = Error;
    fn try_from(queried: Service_) -> Result<Self, Self::Error> {
        Ok(Service {
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
struct Service_ {
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
impl Service_ {
    pub fn delete(id: i32, conn: &PgConnection) -> Result<usize, Error> {
        diesel::delete(dsl::services.filter(dsl::service_id.eq(id)))
            .execute(conn)
            .map_err(|e| e.into())
    }
}

/// New service inserter arguments struct.
#[derive(Insertable, Debug, Default)]
#[table_name = "services"]
pub struct NewService<'a> {
    pub asset_group_id: i32,
    pub name: &'a str,
    pub address: &'a str,
    pub service_type: &'a str,
    pub health_status: &'a str,
    pub input_config_id: Option<i32>,
    pub output_config_id: Option<i32>,
    pub processor_config_id: Option<i32>,
}
impl<'a> NewService<'a> {
    pub fn insert(&'a self, conn: &PgConnection) -> Result<Service, Error> {
        let query = diesel::insert_into(services::table).values(self);
        println!("{}", diesel::debug_query::<Pg, _>(&query));
        let result: Result<Service_, Error> = query.get_result(conn).map_err(|e| e.into());
        Service::try_from(result?)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::asset_groups::temp_asset_group_test;
    use crate::models::*;
    use diesel::PgConnection;

    #[test]
    fn create_input_service() {
        temp_asset_group_test(|conn: &PgConnection, asset_group: &AssetGroup| {
            let input = NewService {
                asset_group_id: asset_group.asset_group_id,
                name: "test_service",
                address: "localhost:2222",
                service_type: "input",
                health_status: "healthy",
                ..Default::default()
            }
            .insert(conn)?;
            println!("Inserted {:#?}", input);

            let service = Service::select(input.service_id, conn);
            println!("Queried {:#?}", service);
            Ok(())
        })
        .unwrap();
    }
}
