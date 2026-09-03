//! Request-edge string hygiene (Symfony-style input layer).
//!
//! Rejects NUL in client-supplied strings once at the HTTP boundary. Persistence
//! adapters use parameterized queries; they do not re-check every bind.

use serenade_contracts::reject_unsafe_sql_param;

use crate::error::ApiError;

/// Rejects NUL in a request string.
pub fn ensure_request_param(value: &str) -> Result<&str, ApiError> {
    reject_unsafe_sql_param(value).map_err(|error| ApiError::from_persist(&error))
}

/// Optional request string.
pub fn ensure_request_param_opt(value: Option<&str>) -> Result<Option<&str>, ApiError> {
    match value {
        None => Ok(None),
        Some(value) => Ok(Some(ensure_request_param(value)?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_nul() {
        assert!(ensure_request_param("a\0b").is_err());
    }

    #[test]
    fn accepts_clean() {
        assert_eq!(ensure_request_param("ok").unwrap(), "ok");
    }
}
