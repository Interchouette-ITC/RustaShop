//! Actix-web API surface.

mod carts;
mod checkout;
mod error;
mod health;
mod openapi;
mod products;
mod request_param;

use actix_web::{web, App, HttpServer};
use tracing::info;

pub use carts::{
    add_cart_line, create_cart, delete_cart_line, get_cart, update_cart_line, CartLineResponse,
    CartResponse, MoneyResponse,
};
pub use checkout::{place_order, OrderLineResponse, OrderResponse};
pub use health::{healthz, HealthResponse};
pub use openapi::{openapi_json, swagger_ui, ApiDoc};
pub use products::{
    get_product, list_products, ProductDetailResponse, ProductListResponse, ProductResponse,
    ProductVariantResponse,
};

/// Default bind address when `RUSTASHOP_BIND` is unset.
pub const DEFAULT_BIND: &str = "127.0.0.1:8080";

/// Environment variable for the API listen address.
pub const BIND_ENV: &str = "RUSTASHOP_BIND";

/// Compile-time persistence backend label for startup logs.
#[cfg(feature = "persist-sqlx")]
const PERSIST_BACKEND: &str = "sqlx";

/// Compile-time persistence backend label for startup logs.
#[cfg(feature = "persist-seaorm")]
const PERSIST_BACKEND: &str = "seaorm";

/// Returns the bind address from `RUSTASHOP_BIND` or [`DEFAULT_BIND`].
#[must_use]
pub fn bind_address() -> String {
    std::env::var(BIND_ENV).unwrap_or_else(|_| DEFAULT_BIND.to_owned())
}

/// Registers HTTP routes on `cfg`.
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthz)
        .service(openapi_json)
        .service(swagger_ui())
        .service(list_products)
        .service(get_product)
        .service(create_cart)
        .service(get_cart)
        .service(add_cart_line)
        .service(update_cart_line)
        .service(delete_cart_line)
        .service(place_order);
}

/// Redacts the password in a Postgres URL for safe logging.
fn redacted_database_url(raw: &str) -> String {
    let Some((scheme, rest)) = raw.split_once("://") else {
        return "(unrecognized DATABASE_URL)".to_owned();
    };
    let Some((userinfo, host_and_path)) = rest.split_once('@') else {
        return format!("{scheme}://{rest}");
    };
    let user = userinfo.split(':').next().unwrap_or(userinfo);
    format!("{scheme}://{user}:***@{host_and_path}")
}

/// Maps bind failures to a clearer message (especially address-in-use).
fn bind_error(bind: &str, error: &std::io::Error) -> std::io::Error {
    if error.kind() == std::io::ErrorKind::AddrInUse {
        return std::io::Error::new(
            error.kind(),
            format!(
                "cannot bind {bind}: address already in use \
                 (another rustashop-api?). Free the port or set {BIND_ENV}"
            ),
        );
    }
    std::io::Error::new(error.kind(), format!("cannot bind {bind}: {error}"))
}

/// Starts the Actix HTTP server on [`bind_address`].
///
/// Loads the catalog repository from `DATABASE_URL` via
/// [`rustashop_persist::catalog_from_env`], then serves HTTP until shutdown.
///
/// # Errors
///
/// Returns [`std::io::Error`] when:
/// - `DATABASE_URL` is missing or the database is unreachable
///   ([`rustashop_persist::MigrateError`] mapped via `std::io::Error::other`)
/// - binding [`bind_address`] fails
/// - the server accept loop fails
#[allow(clippy::future_not_send)]
pub async fn run() -> std::io::Result<()> {
    let bind = bind_address();
    let version = env!("CARGO_PKG_VERSION");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "(unset)".to_owned());

    info!("rustashop API {version}");
    info!("bind: http://{bind} (override with {BIND_ENV})");
    info!("persist: {PERSIST_BACKEND}");
    info!("database: {}", redacted_database_url(&database_url));
    info!("health: http://{bind}/healthz");
    info!("openapi: http://{bind}/openapi.json");
    info!("swagger: http://{bind}/swagger-ui/");

    info!("connecting catalog repository...");
    let catalog = rustashop_persist::catalog_from_env()
        .await
        .map_err(std::io::Error::other)?;
    info!("catalog repository ready");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(catalog.clone()))
            .configure(routes)
    })
    .bind(&bind)
    .map_err(|error| bind_error(&bind, &error))?;

    info!("listening on http://{bind}");
    server.run().await
}

#[cfg(test)]
mod redact_tests {
    use super::redacted_database_url;

    #[test]
    fn redacts_database_password() {
        let raw = "postgres://rustashop:secret@127.0.0.1:5432/rustashop";
        assert_eq!(
            redacted_database_url(raw),
            "postgres://rustashop:***@127.0.0.1:5432/rustashop"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn healthz_returns_ok_json() {
        let app = test::init_service(App::new().configure(routes)).await;
        let req = test::TestRequest::get().uri("/healthz").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: HealthResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "ok");
        assert_eq!(body.kernel, rustashop::kernel_status());
    }

    #[actix_web::test]
    async fn swagger_ui_serves_html() {
        let app = test::init_service(App::new().configure(routes)).await;
        let req = test::TestRequest::get().uri("/swagger-ui/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn openapi_json_lists_product_paths() {
        let app = test::init_service(App::new().configure(routes)).await;
        let req = test::TestRequest::get().uri("/openapi.json").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        let paths = body.get("paths").expect("paths");
        assert!(paths.get("/v1/products").is_some());
        assert!(paths.get("/v1/carts").is_some());
        assert!(paths.get("/v1/checkout").is_some());
        assert!(paths.get("/healthz").is_some());
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
