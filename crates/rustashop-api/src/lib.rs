//! Actix-web API surface.

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

/// Default bind address when `RUSTASHOP_BIND` is unset.
pub const DEFAULT_BIND: &str = "127.0.0.1:8080";

/// Environment variable for the API listen address.
pub const BIND_ENV: &str = "RUSTASHOP_BIND";

/// JSON body for `GET /healthz`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    /// Liveness status.
    pub status: String,
}

/// Returns the bind address from `RUSTASHOP_BIND` or [`DEFAULT_BIND`].
#[must_use]
pub fn bind_address() -> String {
    std::env::var(BIND_ENV).unwrap_or_else(|_| DEFAULT_BIND.to_owned())
}

/// `GET /healthz` handler.
#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_owned(),
    })
}

/// Starts the Actix HTTP server on [`bind_address`].
///
/// # Errors
///
/// Returns an error when binding or serving fails.
#[allow(clippy::future_not_send)]
pub async fn run() -> std::io::Result<()> {
    let bind = bind_address();
    HttpServer::new(|| App::new().service(healthz))
        .bind(bind)?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn healthz_returns_ok_json() {
        let app = test::init_service(App::new().service(healthz)).await;
        let req = test::TestRequest::get().uri("/healthz").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: HealthResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "ok");
    }
}

#[cfg(test)]
mod domain_smoke {
    #[test]
    fn money_compiles_in_api_crate() {
        let currency = rustashop_domain::Currency::new("EUR").expect("EUR");
        let money = rustashop_domain::Money::new(2500, currency);
        assert_eq!(money.amount_minor, 2500);
        assert_eq!(money.currency.as_str(), "EUR");
    }
}
