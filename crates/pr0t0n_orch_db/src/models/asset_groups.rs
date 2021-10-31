use crate::{
    err::Error,
    schema::asset_groups::{self, dsl},
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

#[derive(Queryable, Debug)]
pub struct AssetGroup {
    pub asset_group_id: i32,
    pub name: String,
    pub description: String,
}
impl AssetGroup {
    pub fn delete(id: i32, conn: &PgConnection) -> Result<usize, Error> {
        diesel::delete(dsl::asset_groups.filter(dsl::asset_group_id.eq(id)))
            .execute(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable, Debug)]
#[table_name = "asset_groups"]
pub struct NewAssetGroup<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
impl<'a> NewAssetGroup<'a> {
    pub fn insert(&'a self, conn: &PgConnection) -> Result<AssetGroup, Error> {
        diesel::insert_into(asset_groups::table)
            .values(self)
            .get_result(conn)
            .map_err(|e| e.into())
    }
}

/// Runs a test function `f` using a temporary asset group that is cleaned up
/// regardless of the internal test result.
#[cfg(test)]
pub fn temp_asset_group_test(
    f: fn(&PgConnection, &AssetGroup) -> Result<(), Error>,
) -> Result<(), Error> {
    use crate::establish_connection;

    let conn = establish_connection();
    let asset_group = NewAssetGroup {
        name: "temp_asset_group",
        description: "A test asset group",
    }
    .insert(&conn)?;
    println!("Inserted {:#?}", asset_group);
    let result = f(&conn, &asset_group);

    let num_deleted = AssetGroup::delete(asset_group.asset_group_id, &conn).unwrap();
    println!("Deleted {} asset group.", num_deleted);
    result
}

#[cfg(test)]
mod tests {
    use crate::models::asset_groups::temp_asset_group_test;

    #[test]
    fn test_asset_group() {
        temp_asset_group_test(|_conn, _asset_group| Ok(())).unwrap();
    }
}
