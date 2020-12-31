use crate::app::model::relation_tuple::parser::RelationTupleParserError;
use actix_web::{dev::HttpResponseBuilder, error, http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Bad Request: {0}")]
    BadRequestError(String),
    #[error(transparent)]
    BadRequestRelationTupleParseError(#[from] RelationTupleParserError),
    #[error("UnauthorizedError: {0}")]
    UnauthorizedError(String),
    #[error("Forbidden: {0}")]
    ForbiddenError(String),
    #[error("Not Found: {0}")]
    NotFoundError(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequestError(_) => StatusCode::BAD_REQUEST,
            Self::BadRequestRelationTupleParseError(_) => StatusCode::BAD_REQUEST,
            Self::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
            Self::ForbiddenError(_) => StatusCode::FORBIDDEN,
            Self::NotFoundError(_) => StatusCode::NOT_FOUND,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(ErrorResponse::from(self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

impl ErrorResponse {
    fn from(error: &Error) -> Self {
        Self {
            error: error.to_string(),
            code: error.status_code().as_u16(),
        }
    }
}
