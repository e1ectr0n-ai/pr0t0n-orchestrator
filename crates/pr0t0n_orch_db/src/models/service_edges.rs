use crate::{errors::Error, models::generic::*, schema::service_edges};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl};

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
    pub fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error> {
        let results: Vec<Self> = service_edges::table
            .filter(service_edges::asset_group_id.eq(asset_group_id))
            .get_results(conn)?;
        Ok(results)
    }

    pub fn delete_outputs(conn: &PgConnection, input_service_id: i32) -> Result<usize, Error> {
        let result: usize = diesel::delete(
            service_edges::dsl::service_edges
                .filter(service_edges::input_service_id.eq(input_service_id)),
        )
        .execute(conn)?;
        Ok(result)
    }
}
