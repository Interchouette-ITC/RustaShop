//! HTTP error mapping for the Actix API.

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serenade_contracts::PersistenceError;

/// JSON error body.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorBody {
    /// Machine-readable error.
    pub error: String,
}

/// Handler failure mapped to an HTTP status.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Requested aggregate is missing.
    #[error("not found")]
    NotFound,
    /// Persistence or unexpected failure.
    #[error("internal error")]
    Internal,
}

impl ApiError {
    pub(crate) const fn from_persist(error: &PersistenceError) -> Self {
        match error {
            PersistenceError::NotFound { .. } | PersistenceError::InvalidInput { .. } => {
                Self::NotFound
            }
            PersistenceError::Conflict { .. } | PersistenceError::Internal { .. } => Self::Internal,
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            error: self.to_string(),
        })
    }
}
