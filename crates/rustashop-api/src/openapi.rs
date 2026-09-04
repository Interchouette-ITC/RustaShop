//! `OpenAPI` document and Swagger UI for the Actix API.

use actix_web::{get, HttpResponse, Responder};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::admin_orders::{OrderListResponse, PatchOrderStatusRequest};
use crate::carts::{
    AddCartLineRequest, CartLineResponse, CartResponse, CreateCartRequest, MoneyResponse,
    UpdateCartLineRequest,
};
use crate::checkout::{CheckoutRequest, OrderLineResponse, OrderResponse};
use crate::error::ErrorBody;
use crate::health::HealthResponse;
use crate::products::{
    ProductDetailResponse, ProductListResponse, ProductResponse, ProductVariantResponse,
};

struct AdminSecurityAddon;

impl Modify for AdminSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "admin_bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("token")
                        .build(),
                ),
            );
        }
    }
}

/// Generated `OpenAPI` document.
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::health::healthz,
        crate::products::list_products,
        crate::products::get_product,
        crate::carts::create_cart,
        crate::carts::get_cart,
        crate::carts::add_cart_line,
        crate::carts::update_cart_line,
        crate::carts::delete_cart_line,
        crate::checkout::place_order,
        crate::admin_orders::list_admin_orders,
        crate::admin_orders::patch_admin_order,
        openapi_json
    ),
    components(schemas(
        HealthResponse,
        ProductResponse,
        ProductDetailResponse,
        ProductVariantResponse,
        ProductListResponse,
        CartResponse,
        CartLineResponse,
        MoneyResponse,
        CreateCartRequest,
        AddCartLineRequest,
        UpdateCartLineRequest,
        CheckoutRequest,
        OrderResponse,
        OrderLineResponse,
        OrderListResponse,
        PatchOrderStatusRequest,
        ErrorBody
    )),
    modifiers(&AdminSecurityAddon)
)]
pub struct ApiDoc;

/// `GET /openapi.json` handler.
#[utoipa::path(
    get,
    path = "/openapi.json",
    responses((status = 200, description = "OpenAPI document"))
)]
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    HttpResponse::Ok().json(ApiDoc::openapi())
}

/// Swagger UI at `/swagger-ui/`, pointed at [`openapi_json`].
#[must_use]
pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi.json", ApiDoc::openapi())
}
