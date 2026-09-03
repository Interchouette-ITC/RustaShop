//! Catalog product read handlers.

use actix_web::{get, web, HttpResponse};
use rustashop_domain::Product;
use rustashop_persist::CatalogRepository;
use serde::{Deserialize, Serialize};
use serenade_contracts::{PageRequest, ProductRepository};
use utoipa::{IntoParams, ToSchema};

use crate::error::{ApiError, ErrorBody};
use crate::request_param::ensure_request_param;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

/// Query string for `GET /v1/products`.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListProductsQuery {
    /// Maximum rows (capped).
    pub limit: Option<u32>,
    /// Rows to skip.
    pub offset: Option<u32>,
}

/// Product JSON returned by catalog routes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ProductResponse {
    /// Stable identifier.
    pub id: String,
    /// Optional category id.
    pub category_id: Option<String>,
    /// Unique URL slug.
    pub slug: String,
    /// Display name.
    pub name: String,
    /// Optional long description.
    pub description: Option<String>,
    /// Whether the product is offered for sale.
    pub enabled: bool,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            category_id: product.category_id,
            slug: product.slug,
            name: product.name,
            description: product.description,
            enabled: product.enabled,
        }
    }
}

/// List payload for `GET /v1/products`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ProductListResponse {
    /// Page of products.
    pub items: Vec<ProductResponse>,
}

fn page_request(query: &ListProductsQuery) -> PageRequest {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query.offset.unwrap_or(0);
    PageRequest { limit, offset }
}

/// Lists enabled products.
#[utoipa::path(
    get,
    path = "/v1/products",
    params(ListProductsQuery),
    responses((status = 200, description = "Product page", body = ProductListResponse))
)]
#[get("/v1/products")]
pub async fn list_products(
    catalog: web::Data<CatalogRepository>,
    query: web::Query<ListProductsQuery>,
) -> Result<HttpResponse, ApiError> {
    let items = ProductRepository::list(catalog.get_ref(), page_request(&query))
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    Ok(HttpResponse::Ok().json(ProductListResponse {
        items: items.into_iter().map(ProductResponse::from).collect(),
    }))
}

/// Returns one product by id.
#[utoipa::path(
    get,
    path = "/v1/products/{id}",
    params(("id" = String, Path, description = "Product id")),
    responses(
        (status = 200, description = "Product", body = ProductResponse),
        (status = 404, description = "Unknown id", body = ErrorBody)
    )
)]
#[get("/v1/products/{id}")]
pub async fn get_product(
    catalog: web::Data<CatalogRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    ensure_request_param(&id)?;
    let product = ProductRepository::find_by_id(catalog.get_ref(), &id)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    product.map_or_else(
        || Err(ApiError::NotFound),
        |product| Ok(HttpResponse::Ok().json(ProductResponse::from(product))),
    )
}
