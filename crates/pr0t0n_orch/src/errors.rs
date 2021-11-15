use actix_web::{self, error::ResponseError, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    R2D2(r2d2::Error),
    ActixWeb(actix_web::Error),
    Unknown(String),
    ValidationError,
    BadRequest(String),
    InternalServerError(String),
    Pr0t0nDbError(pr0t0n_orch_db::Error),
    NotFound(String),
    UnprocessableEntity(String),
    BlockingError(String),
    Forbidden,
}
impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self::R2D2(e)
    }
}
impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Self::ActixWeb(e)
    }
}
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::Pr0t0nDbError(e.into())
    }
}
impl From<pr0t0n_orch_db::Error> for Error {
    fn from(e: pr0t0n_orch_db::Error) -> Self {
        Self::Pr0t0nDbError(e)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}
impl From<&str> for ErrorResponse {
    fn from(error: &str) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}
impl From<Vec<String>> for ErrorResponse {
    fn from(error: Vec<String>) -> Self {
        ErrorResponse { errors: error }
    }
}
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            Error::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            Error::UnprocessableEntity(message) => {
                HttpResponse::UnprocessableEntity().json::<ErrorResponse>(message.into())
            }
            Error::Forbidden => HttpResponse::Forbidden().json::<ErrorResponse>("Forbidden".into()),
            _ => {
                error!("Internal server error: {:?}", self);
                HttpResponse::InternalServerError()
                    .json::<ErrorResponse>("Internal Server Error".into())
            }
        }
    }
}
