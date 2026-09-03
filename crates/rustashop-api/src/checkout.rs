//! Checkout HTTP handler.

use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{post, web, FromRequest, HttpRequest, HttpResponse};
use rustashop_domain::Order;
use rustashop_persist::CatalogRepository;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::carts::MoneyResponse;
use crate::error::{ApiError, ErrorBody};
use crate::request_param::{ensure_request_param, ensure_request_param_opt};

/// Body for `POST /v1/checkout`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckoutRequest {
    /// Cart to convert into an order.
    pub cart_id: String,
}

/// Order line JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct OrderLineResponse {
    /// Line id.
    pub id: String,
    /// Variant id when still known.
    pub variant_id: Option<String>,
    /// Quantity.
    pub quantity: i32,
    /// Snapshotted unit price.
    pub unit_price: MoneyResponse,
    /// Snapshotted line total.
    pub line_total: MoneyResponse,
    /// Snapshotted product name.
    pub product_name: String,
    /// Snapshotted SKU.
    pub variant_sku: String,
}

/// Order JSON returned by checkout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct OrderResponse {
    /// Order id.
    pub id: String,
    /// Human-readable number.
    pub number: String,
    /// Source cart id.
    pub cart_id: Option<String>,
    /// Fulfillment state.
    pub state: String,
    /// Payment stub (`pending` until a PSP is wired).
    pub payment_status: String,
    /// Order currency.
    pub currency: String,
    /// Sum of line totals.
    pub items_total: MoneyResponse,
    /// Payable total.
    pub total: MoneyResponse,
    /// Order lines.
    pub lines: Vec<OrderLineResponse>,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            number: order.number,
            cart_id: order.cart_id,
            state: order.state,
            payment_status: order.payment_status,
            currency: order.currency.as_str().to_owned(),
            items_total: MoneyResponse {
                amount_minor: order.items_total.amount_minor,
                currency: order.items_total.currency.as_str().to_owned(),
            },
            total: MoneyResponse {
                amount_minor: order.total.amount_minor,
                currency: order.total.currency.as_str().to_owned(),
            },
            lines: order
                .lines
                .into_iter()
                .map(|line| OrderLineResponse {
                    id: line.id,
                    variant_id: line.variant_id,
                    quantity: line.quantity,
                    unit_price: MoneyResponse {
                        amount_minor: line.unit_price.amount_minor,
                        currency: line.unit_price.currency.as_str().to_owned(),
                    },
                    line_total: MoneyResponse {
                        amount_minor: line.line_total.amount_minor,
                        currency: line.line_total.currency.as_str().to_owned(),
                    },
                    product_name: line.product_name,
                    variant_sku: line.variant_sku,
                })
                .collect(),
        }
    }
}

struct IdempotencyKey(Option<String>);

impl FromRequest for IdempotencyKey {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(Self(idempotency_key(request))))
    }
}

fn idempotency_key(request: &HttpRequest) -> Option<String> {
    request
        .headers()
        .get("idempotency-key")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

/// Converts a cart into a placed order.
#[utoipa::path(
    post,
    path = "/v1/checkout",
    request_body = CheckoutRequest,
    params(("Idempotency-Key" = Option<String>, Header, description = "Replay token")),
    responses(
        (status = 201, description = "Order placed", body = OrderResponse),
        (status = 404, description = "Unknown cart", body = ErrorBody),
        (status = 409, description = "Cart already checked out", body = ErrorBody),
        (status = 422, description = "Empty cart", body = ErrorBody)
    )
)]
#[post("/v1/checkout")]
pub async fn place_order(
    store: web::Data<CatalogRepository>,
    key: IdempotencyKey,
    body: web::Json<CheckoutRequest>,
) -> Result<HttpResponse, ApiError> {
    ensure_request_param(&body.cart_id)?;
    let key = ensure_request_param_opt(key.0.as_deref())?;
    let order = store
        .checkout_cart(&body.cart_id, key)
        .await
        .map_err(|error| ApiError::from_persist(&error))?;
    Ok(HttpResponse::Created().json(OrderResponse::from(order)))
}
