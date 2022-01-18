use serde::{Deserialize, Serialize};

use crate::{models::generic::*, schema::asset_groups};

#[derive(Queryable, Debug)]
pub struct AssetGroup {
    pub asset_group_id: i32,
    pub name: String,
    pub description: String,
}
impl DbUpdate for AssetGroup {
    type Table = asset_groups::table;
}
impl DbDelete for AssetGroup {
    type Table = asset_groups::table;
}
impl DbFind for AssetGroup {
    type Table = asset_groups::table;
}

#[derive(Insertable, Debug)]
#[table_name = "asset_groups"]
pub struct NewAssetGroup<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

impl<'a> DbInsert for NewAssetGroup<'a> {
    type Table = asset_groups::table;
    type Return = AssetGroup;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetGroupRequest {
    pub asset_group_id: i32,
}

#[cfg(test)]
mod tests {
    use crate::testing::temp_asset_group_test;

    #[test]
    fn test_asset_group() {
        temp_asset_group_test(|_conn, _asset_group| Ok(())).unwrap();
    }
}
