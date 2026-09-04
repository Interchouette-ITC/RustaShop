//! Admin product list.

use actix_web::{get, web, HttpResponse};
use rustashop_persist::CatalogRepository;
use serde::Deserialize;
use serenade_contracts::PageRequest;
use utoipa::IntoParams;

use crate::admin_auth::{AdminAuthConfig, AdminBearer};
use crate::error::{ApiError, ErrorBody};
use crate::products::{ProductListResponse, ProductResponse};

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

/// Query string for `GET /v1/admin/products`.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListAdminProductsQuery {
    /// Maximum rows (capped).
    pub limit: Option<u32>,
    /// Rows to skip.
    pub offset: Option<u32>,
}

/// `GET /v1/admin/products` - list products including disabled (bearer required).
#[utoipa::path(
    get,
    path = "/v1/admin/products",
    params(ListAdminProductsQuery),
    security(("admin_bearer" = [])),
    responses(
        (status = 200, description = "Product page", body = ProductListResponse),
        (status = 401, description = "Missing or invalid bearer", body = ErrorBody)
    )
)]
#[get("/v1/admin/products")]
pub async fn list_admin_products(
    bearer: AdminBearer,
    auth: web::Data<AdminAuthConfig>,
    catalog: web::Data<CatalogRepository>,
    query: web::Query<ListAdminProductsQuery>,
) -> Result<HttpResponse, ApiError> {
    auth.authorize_bearer(bearer.0.as_deref())?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query.offset.unwrap_or(0);
    let page = PageRequest { limit, offset };
    let products = catalog
        .list_all_products(page)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    Ok(HttpResponse::Ok().json(ProductListResponse {
        items: products.into_iter().map(ProductResponse::from).collect(),
    }))
}
