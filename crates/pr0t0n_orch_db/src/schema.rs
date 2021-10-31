table! {
    asset_groups (asset_group_id) {
        asset_group_id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    input_configs (input_config_id) {
        input_config_id -> Int4,
        asset_group_id -> Int4,
        name -> Varchar,
        description -> Text,
        max_sample_rate -> Float4,
        json_config -> Nullable<Text>,
    }
}

table! {
    output_configs (output_config_id) {
        output_config_id -> Int4,
        asset_group_id -> Int4,
        name -> Varchar,
        description -> Text,
        json_config -> Text,
    }
}

table! {
    processor_configs (processor_config_id) {
        processor_config_id -> Int4,
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
        input_config_id -> Nullable<Int4>,
        output_config_id -> Nullable<Int4>,
        processor_config_id -> Nullable<Int4>,
    }
}

joinable!(input_configs -> asset_groups (asset_group_id));
joinable!(output_configs -> asset_groups (asset_group_id));
joinable!(processor_configs -> asset_groups (asset_group_id));
joinable!(services -> asset_groups (asset_group_id));
joinable!(services -> input_configs (input_config_id));
joinable!(services -> output_configs (output_config_id));
joinable!(services -> processor_configs (processor_config_id));

allow_tables_to_appear_in_same_query!(
    asset_groups,
    input_configs,
    output_configs,
    processor_configs,
    service_edges,
    services,
);
