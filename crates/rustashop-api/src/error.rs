//! HTTP error mapping for the Actix API.

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use rustashop_domain::DomainError;
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
    /// Request body or domain rule rejected.
    #[error("{0}")]
    Unprocessable(String),
    /// Persistence or unexpected failure.
    #[error("internal error")]
    Internal,
}

impl ApiError {
    pub(crate) fn from_persist(error: &PersistenceError) -> Self {
        match error {
            PersistenceError::NotFound { .. } => Self::NotFound,
            PersistenceError::InvalidInput { message } => Self::Unprocessable(message.clone()),
            PersistenceError::Conflict { .. } | PersistenceError::Internal { .. } => Self::Internal,
        }
    }

    pub(crate) fn from_domain(error: &DomainError) -> Self {
        match error {
            DomainError::InvalidCurrency(_)
            | DomainError::CurrencyMismatch { .. }
            | DomainError::InvalidQuantity(_)
            | DomainError::Overflow => Self::Unprocessable(error.to_string()),
            DomainError::LineNotFound(_) => Self::NotFound,
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            error: self.to_string(),
        })
    }
}
