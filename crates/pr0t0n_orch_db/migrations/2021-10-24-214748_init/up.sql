-- Groups of assets for user visibility.
CREATE TABLE asset_groups (
  asset_group_id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL
);

-- Defined in own table to so services can re-use a config.
CREATE TABLE input_configs (
  input_config_id SERIAL PRIMARY KEY,
  asset_group_id SERIAL NOT NULL REFERENCES asset_groups(asset_group_id) ON DELETE CASCADE,

  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  max_sample_rate REAL NOT NULL,
  json_config TEXT
);

-- Defined in own table to so services can re-use a config.
CREATE TABLE output_configs (
  output_config_id SERIAL PRIMARY KEY,
  asset_group_id SERIAL NOT NULL REFERENCES asset_groups(asset_group_id) ON DELETE CASCADE,

  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  json_config TEXT NOT NULL
);

-- Defined in own table to so services can re-use a config.
CREATE TABLE processor_configs (
  processor_config_id SERIAL PRIMARY KEY,
  asset_group_id SERIAL NOT NULL REFERENCES asset_groups(asset_group_id) ON DELETE CASCADE,

  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  json_config TEXT NOT NULL
);

-- Service. Respresents a processing server, probably hosted in the cloud.
CREATE TABLE services (
  service_id SERIAL PRIMARY KEY,
  asset_group_id SERIAL NOT NULL REFERENCES asset_groups(asset_group_id) ON DELETE CASCADE,

  name VARCHAR(255) NOT NULL,
  address VARCHAR(255) NOT NULL,
  service_type VARCHAR(255) CHECK (service_type IN ('input', 'output', 'processor')) NOT NULL,
  health_status VARCHAR(255) CHECK (health_status IN ('healthy', 'disconnected', 'warning', 'critical')) NOT NULL,

  -- Config IDs will be null for all except the one corresponding to this service type.
  input_config_id INT DEFAULT (NULL) REFERENCES input_configs(input_config_id) ON DELETE SET NULL,
  output_config_id INT DEFAULT (NULL) REFERENCES output_configs(output_config_id) ON DELETE SET NULL,
  processor_config_id INT DEFAULT (NULL) REFERENCES processor_configs(processor_config_id) ON DELETE SET NULL
);

-- Connections between services.
CREATE TABLE service_edges (
  input_service_id SERIAL REFERENCES services(service_id) ON DELETE CASCADE,
  output_service_id SERIAL REFERENCES services(service_id) ON DELETE CASCADE,
  CONSTRAINT service_edge_pkey PRIMARY KEY (input_service_id, output_service_id),
  asset_group_id SERIAL NOT NULL REFERENCES asset_groups(asset_group_id) ON DELETE CASCADE
);