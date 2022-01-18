table! {
    asset_groups (asset_group_id) {
        asset_group_id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    configs (config_id) {
        config_id -> Int4,
        asset_group_id -> Int4,
        name -> Varchar,
        description -> Text,
        json_config -> Text,
    }
}

table! {
    service_edges (input_service_id, output_service_id) {
        input_service_id -> Int4,
        output_service_id -> Int4,
        asset_group_id -> Int4,
    }
}

table! {
    services (service_id) {
        service_id -> Int4,
        asset_group_id -> Int4,
        name -> Varchar,
        address -> Varchar,
        service_type -> Varchar,
        health_status -> Varchar,
        config_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(configs -> asset_groups (asset_group_id));
joinable!(service_edges -> asset_groups (asset_group_id));
joinable!(services -> asset_groups (asset_group_id));
joinable!(services -> configs (config_id));

allow_tables_to_appear_in_same_query!(
    asset_groups,
    configs,
    service_edges,
    services,
    users,
);
