/// Error enum.
#[derive(Debug)]
pub enum Error {
    UnknownError,
    DatabaseError(diesel::result::Error),
    InvalidEnumValue(String),
}
impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::DatabaseError(err)
    }
}
