use crate::{err::Error, models::*};
use diesel::PgConnection;

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

    let num_deleted = AssetGroup::delete(&conn, asset_group.asset_group_id).unwrap();
    println!("Deleted {} asset group.", num_deleted);
    result
}
