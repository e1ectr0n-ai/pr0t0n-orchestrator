use actix_web::{self, error::ResponseError, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    ActixWeb(actix_web::Error),
    Unknown(String),
    ValidationError,
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    UnprocessableEntity(String),
    BlockingError(String),
    Forbidden,
}
impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Self::ActixWeb(e)
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
