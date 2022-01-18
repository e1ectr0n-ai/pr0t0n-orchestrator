use std::collections::HashMap;

use diesel::PgConnection;

use crate::Error;

pub trait Asset: Sized {
    /// Gets the identifier string for this item.
    fn get_string_id(&self) -> &str;

    /// Gets a vector for all entities in the asset group.
    fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error>;

    /// Construct a map for the entites.
    fn get_group_map(
        conn: &PgConnection,
        asset_group_id: i32,
    ) -> Result<HashMap<String, Self>, Error> {
        // Load into map.
        let mut vec = Self::get_group(conn, asset_group_id)?;
        let mut map: HashMap<String, Self> = HashMap::new();
        for asset in vec.drain(..) {
            map.insert(asset.get_string_id().to_string(), asset);
        }
        Ok(map)
    }
}

/// More ergonic representation of an asset used for syncable configs entered by the user.
pub trait AssetRepr<'a>: Sized {
    /// The type in the database this represents.
    type Asset: Asset;

    /// Gets the identifier string for this item.
    fn get_string_id(&self) -> &str;

    /// Merge this type into the asset changeset.
    fn try_merge_asset(&self, asset: &mut Self::Asset) -> Result<(), Error>;

    /// Constructs the tuple of `(to_insert, to_update, to_delete)` from a list of representations
    /// and a map of changesets for existing elements in the database.
    fn partition_diff(
        mut existing: HashMap<String, Self::Asset>,
        reprs: &mut Vec<Self>,
    ) -> Result<(Vec<Self>, Vec<(Self, Self::Asset)>, Vec<Self::Asset>), Error> {
        let mut to_insert: Vec<Self> = Vec::new();
        let mut to_update: Vec<(Self, Self::Asset)> = Vec::new();
        for repr in reprs.drain(..) {
            if let Some(mut asset) = existing.remove(repr.get_string_id()) {
                repr.try_merge_asset(&mut asset)?;
                to_update.push((repr, asset));
            } else {
                to_insert.push(repr);
            }
        }
        let to_delete: Vec<Self::Asset> = existing.into_values().collect();
        Ok((to_insert, to_update, to_delete))
    }

    // Syncs a collection of entities to the database.
    fn sync_db(
        conn: &PgConnection,
        asset_group_id: i32,
        reprs: &mut Vec<Self>,
    ) -> Result<(), Error>;

    // Get the asset representation for this asset group.
    fn get_group(conn: &PgConnection, asset_group_id: i32) -> Result<Vec<Self>, Error>;
}
