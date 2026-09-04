//! Admin bearer token gate for `/v1/{admin_api_prefix}/*`.

use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{FromRequest, HttpRequest};

use crate::error::ApiError;

/// Preferred env for the local admin bearer secret.
pub const ADMIN_TOKEN_ENV: &str = "RUSTASHOP_ADMIN_API_TOKEN";

/// Alternate env name from the admin API issue (`ADMIN_API_TOKEN`).
pub const ADMIN_TOKEN_ENV_ALT: &str = "ADMIN_API_TOKEN";

/// Expected admin bearer token (empty rejects all admin calls).
#[derive(Clone, Debug, Default)]
pub struct AdminAuthConfig {
    token: String,
}

impl AdminAuthConfig {
    /// Loads from `RUSTASHOP_ADMIN_API_TOKEN`, then `ADMIN_API_TOKEN`.
    #[must_use]
    pub fn from_env() -> Self {
        let token = std::env::var(ADMIN_TOKEN_ENV)
            .or_else(|_| std::env::var(ADMIN_TOKEN_ENV_ALT))
            .unwrap_or_default();
        Self { token }
    }

    /// Builds a config with an explicit token (tests).
    #[must_use]
    pub fn from_token(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    /// Whether a non-empty token is configured.
    #[must_use]
    pub const fn is_configured(&self) -> bool {
        !self.token.is_empty()
    }

    /// Requires a bearer secret matching the configured token.
    ///
    /// # Errors
    ///
    /// Returns unauthorized when the token is unset, missing, or wrong.
    pub fn authorize_bearer(&self, presented: Option<&str>) -> Result<(), ApiError> {
        if self.token.is_empty() {
            return Err(ApiError::Unauthorized);
        }
        let Some(presented) = presented else {
            return Err(ApiError::Unauthorized);
        };
        if presented != self.token {
            return Err(ApiError::Unauthorized);
        }
        Ok(())
    }
}

/// `Authorization: Bearer …` value extracted for admin routes.
pub struct AdminBearer(pub Option<String>);

impl FromRequest for AdminBearer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(Self(bearer_token(request))))
    }
}

fn bearer_token(request: &HttpRequest) -> Option<String> {
    request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_token_is_configured() {
        let config = AdminAuthConfig::from_token("secret");
        assert!(config.is_configured());
        assert!(config.authorize_bearer(Some("secret")).is_ok());
    }

    #[test]
    fn from_env_reads_preferred_then_alt() {
        // SAFETY: test isolates admin token env keys.
        unsafe {
            std::env::remove_var(ADMIN_TOKEN_ENV);
            std::env::remove_var(ADMIN_TOKEN_ENV_ALT);
        }
        assert!(!AdminAuthConfig::from_env().is_configured());
        unsafe {
            std::env::set_var(ADMIN_TOKEN_ENV_ALT, "alt-secret");
        }
        assert_eq!(AdminAuthConfig::from_env().token, "alt-secret");
        unsafe {
            std::env::set_var(ADMIN_TOKEN_ENV, "preferred");
        }
        assert_eq!(AdminAuthConfig::from_env().token, "preferred");
        unsafe {
            std::env::remove_var(ADMIN_TOKEN_ENV);
            std::env::remove_var(ADMIN_TOKEN_ENV_ALT);
        }
    }

    #[rstest::rstest]
    #[case::unset(AdminAuthConfig::from_token(""), Some("x"))]
    #[case::missing(AdminAuthConfig::from_token("secret"), None)]
    #[case::wrong(AdminAuthConfig::from_token("secret"), Some("nope"))]
    fn authorize_bearer_rejects(#[case] config: AdminAuthConfig, #[case] presented: Option<&str>) {
        assert!(matches!(
            config.authorize_bearer(presented),
            Err(ApiError::Unauthorized)
        ));
    }
}
