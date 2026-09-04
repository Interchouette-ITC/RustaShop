//! Admin bearer token gate for `/v1/admin/*`.

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
