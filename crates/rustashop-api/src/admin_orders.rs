//! Admin order list and status PATCH.

use actix_web::{get, patch, web, HttpResponse};
use rustashop_domain::OrderState;
use rustashop_persist::CatalogRepository;
use serde::{Deserialize, Serialize};
use serenade_contracts::PageRequest;
use utoipa::{IntoParams, ToSchema};

use crate::admin_auth::{AdminAuthConfig, AdminBearer};
use crate::checkout::OrderResponse;
use crate::error::{ApiError, ErrorBody};
use crate::request_param::ensure_request_param;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

/// Query string for `GET /v1/admin/orders`.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListOrdersQuery {
    /// Maximum rows (capped).
    pub limit: Option<u32>,
    /// Rows to skip.
    pub offset: Option<u32>,
}

/// Paginated admin order list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct OrderListResponse {
    /// Orders newest first.
    pub items: Vec<OrderResponse>,
}

/// Body for `PATCH /v1/admin/orders/{id}`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchOrderStatusRequest {
    /// Fulfillment status: `placed`, `paid`, `shipped`, or `cancelled`.
    pub status: String,
}

/// `GET /v1/admin/orders` - list orders (bearer required).
#[utoipa::path(
    get,
    path = "/v1/admin/orders",
    params(ListOrdersQuery),
    security(("admin_bearer" = [])),
    responses(
        (status = 200, description = "Order page", body = OrderListResponse),
        (status = 401, description = "Missing or invalid bearer", body = ErrorBody)
    )
)]
#[get("/v1/admin/orders")]
pub async fn list_admin_orders(
    bearer: AdminBearer,
    auth: web::Data<AdminAuthConfig>,
    catalog: web::Data<CatalogRepository>,
    query: web::Query<ListOrdersQuery>,
) -> Result<HttpResponse, ApiError> {
    auth.authorize_bearer(bearer.0.as_deref())?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query.offset.unwrap_or(0);
    let page = PageRequest { limit, offset };
    let orders = catalog
        .list_orders(page)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    Ok(HttpResponse::Ok().json(OrderListResponse {
        items: orders.into_iter().map(OrderResponse::from).collect(),
    }))
}

/// `PATCH /v1/admin/orders/{id}` - update fulfillment status (bearer required).
#[utoipa::path(
    patch,
    path = "/v1/admin/orders/{id}",
    params(("id" = String, Path, description = "Order id")),
    request_body = PatchOrderStatusRequest,
    security(("admin_bearer" = [])),
    responses(
        (status = 200, description = "Updated order", body = OrderResponse),
        (status = 401, description = "Missing or invalid bearer", body = ErrorBody),
        (status = 404, description = "Order not found", body = ErrorBody),
        (status = 422, description = "Invalid status", body = ErrorBody)
    )
)]
#[patch("/v1/admin/orders/{id}")]
pub async fn patch_admin_order(
    bearer: AdminBearer,
    auth: web::Data<AdminAuthConfig>,
    catalog: web::Data<CatalogRepository>,
    path: web::Path<String>,
    body: web::Json<PatchOrderStatusRequest>,
) -> Result<HttpResponse, ApiError> {
    auth.authorize_bearer(bearer.0.as_deref())?;
    let id = path.into_inner();
    ensure_request_param(&id)?;
    let state = OrderState::parse(&body.status).map_err(|error| ApiError::from_domain(&error))?;
    let order = catalog
        .update_order_state(&id, state)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    Ok(HttpResponse::Ok().json(OrderResponse::from(order)))
}
