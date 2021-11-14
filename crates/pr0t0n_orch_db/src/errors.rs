/// Error enum.
#[derive(Debug)]
pub enum Error {
    UnknownError,
    DatabaseError(diesel::result::Error),
    DatabasePoolError(r2d2::Error),
    InvalidEnumValue(String),
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
