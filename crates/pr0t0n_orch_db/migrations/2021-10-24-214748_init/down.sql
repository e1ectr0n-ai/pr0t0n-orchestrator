-- -- Drop tables that depend on asset_groups.
DROP TABLE IF EXISTS service_edges;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS configs;
DROP TABLE IF EXISTS ouputs;
DROP TABLE IF EXISTS event_logs;
DROP TABLE IF EXISTS asset_groups;
DROP TYPE IF EXISTS servicetype;
DROP TYPE IF EXISTS healthstatus;