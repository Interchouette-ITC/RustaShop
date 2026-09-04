//! Actix-web API surface.

mod carts;
mod checkout;
mod error;
mod health;
mod openapi;
mod products;
mod request_param;

use actix_web::{web, App, HttpServer};

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
    let catalog = rustashop_persist::catalog_from_env()
        .await
        .map_err(std::io::Error::other)?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(catalog.clone()))
            .configure(routes)
    })
    .bind(bind)?
    .run()
    .await
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
