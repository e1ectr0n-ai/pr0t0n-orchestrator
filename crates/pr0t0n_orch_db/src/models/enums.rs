use diesel_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use diesel::sql_types::VarChar;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize, DbEnum,
)]
#[sql_type = "VarChar"]
#[error_fn = "Error::invalid_enum"]
#[error_type = "Error"]
pub enum ServiceType {
    None,
    Input,
    Output,
    Processor,
}
impl Default for ServiceType {
    fn default() -> Self {
        Self::None
    }
}

/// Health status for a service.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize, DbEnum,
)]
#[sql_type = "VarChar"]
#[error_fn = "Error::invalid_enum"]
#[error_type = "Error"]
pub enum HealthStatus {
    Healthy,
    Disconnected,
    Warning,
    Critical,
}
impl Default for HealthStatus {
    fn default() -> Self {
        Self::Healthy
    }
}
