//! Cart HTTP handlers.

use actix_web::{delete, get, patch, post, web, HttpResponse};
use rustashop_domain::{Cart, CartLine, Currency};
use rustashop_persist::CatalogRepository;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::{ApiError, ErrorBody};

/// Body for `POST /v1/carts`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCartRequest {
    /// ISO currency for the cart (default `EUR`).
    pub currency: Option<String>,
}

/// Body for `POST /v1/carts/{id}/lines`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddCartLineRequest {
    /// Variant to add.
    pub variant_id: String,
    /// Quantity greater than zero.
    pub quantity: i32,
}

/// Body for `PATCH /v1/carts/{id}/lines/{line_id}`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCartLineRequest {
    /// Replacement quantity greater than zero.
    pub quantity: i32,
}

/// Money JSON for cart responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct MoneyResponse {
    /// Amount in minor units.
    pub amount_minor: i64,
    /// ISO currency code.
    pub currency: String,
}

/// Cart line JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CartLineResponse {
    /// Line id.
    pub id: String,
    /// Variant id.
    pub variant_id: String,
    /// Quantity.
    pub quantity: i32,
    /// Snapshotted unit price.
    pub unit_price: MoneyResponse,
    /// Line total (`unit_price × quantity`).
    pub line_total: MoneyResponse,
    /// Snapshotted product name.
    pub product_name: String,
    /// Snapshotted SKU.
    pub variant_sku: String,
}

/// Cart JSON including recalculated totals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CartResponse {
    /// Cart id.
    pub id: String,
    /// Optional customer id.
    pub customer_id: Option<String>,
    /// Session token.
    pub token: String,
    /// Cart status (`open` or `checked_out`).
    pub status: String,
    /// Cart currency.
    pub currency: String,
    /// Cart lines.
    pub lines: Vec<CartLineResponse>,
    /// Sum of line totals.
    pub items_total: MoneyResponse,
}

impl CartResponse {
    fn try_from_cart(cart: Cart) -> Result<Self, ApiError> {
        let items_total = cart
            .items_total()
            .map_err(|error| ApiError::from_domain(&error))?;
        let mut lines = Vec::with_capacity(cart.lines.len());
        for line in cart.lines {
            let line_total = line
                .line_total()
                .map_err(|error| ApiError::from_domain(&error))?;
            lines.push(CartLineResponse {
                id: line.id,
                variant_id: line.variant_id,
                quantity: line.quantity,
                unit_price: MoneyResponse {
                    amount_minor: line.unit_price.amount_minor,
                    currency: line.unit_price.currency.as_str().to_owned(),
                },
                line_total: MoneyResponse {
                    amount_minor: line_total.amount_minor,
                    currency: line_total.currency.as_str().to_owned(),
                },
                product_name: line.product_name,
                variant_sku: line.variant_sku,
            });
        }
        Ok(Self {
            id: cart.id,
            customer_id: cart.customer_id,
            token: cart.token,
            status: cart.status.as_str().to_owned(),
            currency: cart.currency.as_str().to_owned(),
            lines,
            items_total: MoneyResponse {
                amount_minor: items_total.amount_minor,
                currency: items_total.currency.as_str().to_owned(),
            },
        })
    }
}

async fn reload_cart(store: &CatalogRepository, id: &str) -> Result<CartResponse, ApiError> {
    let cart = store
        .find_cart_by_id(id)
        .await
        .map_err(|error| ApiError::from_persist(&error))?
        .ok_or(ApiError::NotFound)?;
    CartResponse::try_from_cart(cart)
}

/// Creates an empty cart.
#[utoipa::path(
    post,
    path = "/v1/carts",
    request_body = CreateCartRequest,
    responses(
        (status = 201, description = "Cart created", body = CartResponse),
        (status = 422, description = "Invalid currency", body = ErrorBody)
    )
)]
#[post("/v1/carts")]
pub async fn create_cart(
    store: web::Data<CatalogRepository>,
    body: web::Json<CreateCartRequest>,
) -> Result<HttpResponse, ApiError> {
    let code = body.currency.as_deref().unwrap_or("EUR");
    let currency = Currency::new(code).map_err(|error| ApiError::from_domain(&error))?;
    let cart = store
        .create_cart(&currency)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    Ok(HttpResponse::Created().json(CartResponse::try_from_cart(cart)?))
}

/// Returns one cart by id.
#[utoipa::path(
    get,
    path = "/v1/carts/{id}",
    params(("id" = String, Path, description = "Cart id")),
    responses(
        (status = 200, description = "Cart", body = CartResponse),
        (status = 404, description = "Unknown id", body = ErrorBody)
    )
)]
#[get("/v1/carts/{id}")]
pub async fn get_cart(
    store: web::Data<CatalogRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let body = reload_cart(store.get_ref(), &id).await?;
    Ok(HttpResponse::Ok().json(body))
}

/// Adds a line (merges quantity when the variant is already present).
#[utoipa::path(
    post,
    path = "/v1/carts/{id}/lines",
    params(("id" = String, Path, description = "Cart id")),
    request_body = AddCartLineRequest,
    responses(
        (status = 200, description = "Cart after add", body = CartResponse),
        (status = 404, description = "Unknown cart or variant", body = ErrorBody),
        (status = 422, description = "Invalid quantity or currency", body = ErrorBody)
    )
)]
#[post("/v1/carts/{id}/lines")]
pub async fn add_cart_line(
    store: web::Data<CatalogRepository>,
    path: web::Path<String>,
    body: web::Json<AddCartLineRequest>,
) -> Result<HttpResponse, ApiError> {
    let cart_id = path.into_inner();
    let mut cart = store
        .find_cart_by_id(&cart_id)
        .await
        .map_err(|error| ApiError::from_persist(&error))?
        .ok_or(ApiError::NotFound)?;
    let (variant, product_name) = store
        .find_variant_for_cart(&body.variant_id)
        .await
        .map_err(|error| ApiError::from_persist(&error))?
        .ok_or(ApiError::NotFound)?;
    let line = CartLine::from_variant(
        String::new(),
        cart.id.clone(),
        &variant,
        product_name,
        body.quantity,
    )
    .map_err(|error| ApiError::from_domain(&error))?;
    cart.upsert_line(line)
        .map_err(|error| ApiError::from_domain(&error))?;
    store
        .save_cart(&cart)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    let body = reload_cart(store.get_ref(), &cart_id).await?;
    Ok(HttpResponse::Ok().json(body))
}

/// Updates a line quantity.
#[utoipa::path(
    patch,
    path = "/v1/carts/{id}/lines/{line_id}",
    params(
        ("id" = String, Path, description = "Cart id"),
        ("line_id" = String, Path, description = "Line id")
    ),
    request_body = UpdateCartLineRequest,
    responses(
        (status = 200, description = "Cart after update", body = CartResponse),
        (status = 404, description = "Unknown cart or line", body = ErrorBody),
        (status = 422, description = "Invalid quantity", body = ErrorBody)
    )
)]
#[patch("/v1/carts/{id}/lines/{line_id}")]
pub async fn update_cart_line(
    store: web::Data<CatalogRepository>,
    path: web::Path<(String, String)>,
    body: web::Json<UpdateCartLineRequest>,
) -> Result<HttpResponse, ApiError> {
    let (cart_id, line_id) = path.into_inner();
    let mut cart = store
        .find_cart_by_id(&cart_id)
        .await
        .map_err(|error| ApiError::from_persist(&error))?
        .ok_or(ApiError::NotFound)?;
    cart.update_line_quantity(&line_id, body.quantity)
        .map_err(|error| ApiError::from_domain(&error))?;
    store
        .save_cart(&cart)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    let body = reload_cart(store.get_ref(), &cart_id).await?;
    Ok(HttpResponse::Ok().json(body))
}

/// Removes a line.
#[utoipa::path(
    delete,
    path = "/v1/carts/{id}/lines/{line_id}",
    params(
        ("id" = String, Path, description = "Cart id"),
        ("line_id" = String, Path, description = "Line id")
    ),
    responses(
        (status = 200, description = "Cart after delete", body = CartResponse),
        (status = 404, description = "Unknown cart or line", body = ErrorBody)
    )
)]
#[delete("/v1/carts/{id}/lines/{line_id}")]
pub async fn delete_cart_line(
    store: web::Data<CatalogRepository>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let (cart_id, line_id) = path.into_inner();
    let mut cart = store
        .find_cart_by_id(&cart_id)
        .await
        .map_err(|error| ApiError::from_persist(&error))?
        .ok_or(ApiError::NotFound)?;
    cart.remove_line(&line_id)
        .map_err(|error| ApiError::from_domain(&error))?;
    store
        .save_cart(&cart)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    let body = reload_cart(store.get_ref(), &cart_id).await?;
    Ok(HttpResponse::Ok().json(body))
}
