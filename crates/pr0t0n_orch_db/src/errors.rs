/// Error enum.
#[derive(Debug, Clone)]
pub enum Error {
    UnknownError,
    DotEnvError(String),
    DieselError(String),
    PoolBuildError,
    PoolError(String),
    InteractError(String),
    InvalidEnumValue(String),
}
impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::DieselError(err.to_string())
    }
}
impl From<deadpool_diesel::PoolError> for Error {
    fn from(err: deadpool_diesel::PoolError) -> Self {
        Self::PoolError(err.to_string())
    }
}
impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Self::InteractError(err.to_string())
    }
}
impl From<dotenv::Error> for Error {
    fn from(err: dotenv::Error) -> Self {
        Self::DotEnvError(err.to_string())
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
