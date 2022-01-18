/// Error enum.
#[derive(Debug)]
pub enum Error {
    UnknownError,
    DatabaseError(diesel::result::Error),
    DatabasePoolError(r2d2::Error),
    InvalidEnumValue(String),
    SerdeJsonError(serde_json::Error),
    DatabaseSyncError(String),
}
impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::DatabaseError(err)
    }
}
impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Self::DatabasePoolError(err)
    }
}
impl Error {
    pub fn invalid_enum(msg: String) -> Self {
        Self::InvalidEnumValue(msg)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
    }
}
