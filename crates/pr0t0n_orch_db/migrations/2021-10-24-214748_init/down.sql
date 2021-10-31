-- -- Drop tables that depend on asset_groups.
DROP TABLE IF EXISTS service_edges;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS input_configs;
DROP TABLE IF EXISTS output_configs;
DROP TABLE IF EXISTS processor_configs;
DROP TABLE IF EXISTS ouputs;
DROP TABLE IF EXISTS asset_groups;

DROP TYPE IF EXISTS service_type;
DROP TYPE IF EXISTS health_status;