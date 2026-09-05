//! Admin order list and status PATCH (JSON via Serenade front; `OpenAPI` stubs stay here).

use rustashop_domain::OrderState;
use rustashop_persist::CatalogRepository;
use serde::{Deserialize, Serialize};
use serenade_contracts::PageRequest;
use serenade_http::Response;
use utoipa::{IntoParams, ToSchema};

use crate::admin_auth::AdminAuthConfig;
use crate::checkout::OrderResponse;
use crate::error::{api_error_json_response, json_response, ApiError, ErrorBody};
use crate::request_param::ensure_request_param;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

/// Query string for admin order list.
#[derive(Debug, Default, Deserialize, IntoParams)]
pub struct ListOrdersQuery {
    /// Maximum rows (capped).
    pub limit: Option<u32>,
    /// Rows to skip.
    pub offset: Option<u32>,
}

impl ListOrdersQuery {
    /// Parses `limit` / `offset` from a raw query string (`a=1&b=2`).
    #[must_use]
    pub fn from_query_string(query: Option<&str>) -> Self {
        let Some(query) = query.filter(|value| !value.is_empty()) else {
            return Self::default();
        };
        let mut limit = None;
        let mut offset = None;
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or("");
            if key.is_empty() {
                continue;
            }
            let value = parts.next().unwrap_or("");
            match key {
                "limit" => limit = value.parse().ok(),
                "offset" => offset = value.parse().ok(),
                _ => {}
            }
        }
        Self { limit, offset }
    }
}

/// Paginated admin order list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct OrderListResponse {
    /// Orders newest first.
    pub items: Vec<OrderResponse>,
}

/// Body for admin order status PATCH.
#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchOrderStatusRequest {
    /// Fulfillment status: `placed`, `paid`, `shipped`, or `cancelled`.
    pub status: String,
}

fn page_request(query: &ListOrdersQuery) -> PageRequest {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query.offset.unwrap_or(0);
    PageRequest { limit, offset }
}

fn parse_json_body<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, ApiError> {
    serde_json::from_slice(body).map_err(|error| ApiError::Unprocessable(error.to_string()))
}

/// Lists orders as a Serenade JSON [`Response`].
pub async fn list_admin_orders_response(
    auth: &AdminAuthConfig,
    bearer: Option<&str>,
    catalog: &CatalogRepository,
    query: &ListOrdersQuery,
) -> Response {
    if let Err(error) = auth.authorize_bearer(bearer) {
        return api_error_json_response(&error);
    }
    match catalog.list_orders(page_request(query)).await {
        Ok(orders) => json_response(
            200,
            &OrderListResponse {
                items: orders.into_iter().map(OrderResponse::from).collect(),
            },
        ),
        Err(error) => api_error_json_response(&ApiError::from_persist(&error)),
    }
}

/// Updates order fulfillment status as a Serenade JSON [`Response`].
pub async fn patch_admin_order_response(
    auth: &AdminAuthConfig,
    bearer: Option<&str>,
    catalog: &CatalogRepository,
    order_id: &str,
    body: &[u8],
) -> Response {
    if let Err(error) = auth.authorize_bearer(bearer) {
        return api_error_json_response(&error);
    }
    if let Err(error) = ensure_request_param(order_id) {
        return api_error_json_response(&error);
    }
    let request = match parse_json_body::<PatchOrderStatusRequest>(body) {
        Ok(request) => request,
        Err(error) => return api_error_json_response(&error),
    };
    let state = match OrderState::parse(&request.status) {
        Ok(state) => state,
        Err(error) => return api_error_json_response(&ApiError::from_domain(&error)),
    };
    match catalog.update_order_state(order_id, state).await {
        Ok(order) => json_response(200, &OrderResponse::from(order)),
        Err(error) => api_error_json_response(&ApiError::from_persist(&error)),
    }
}

/// `GET /v1/{admin_api_prefix}/orders` `OpenAPI` path (Serenade front).
#[utoipa::path(
    get,
    path = "/v1/{admin_api_prefix}/orders",
    params(ListOrdersQuery),
    security(("admin_bearer" = [])),
    responses(
        (status = 200, description = "Order page", body = OrderListResponse),
        (status = 401, description = "Missing or invalid bearer", body = ErrorBody)
    )
)]
#[allow(clippy::missing_const_for_fn)]
pub fn list_admin_orders() {}

/// `PATCH /v1/{admin_api_prefix}/orders/{id}` `OpenAPI` path (Serenade front).
#[utoipa::path(
    patch,
    path = "/v1/{admin_api_prefix}/orders/{id}",
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
#[allow(clippy::missing_const_for_fn)]
pub fn patch_admin_order() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_list_query_string() {
        let query = ListOrdersQuery::from_query_string(Some("limit=3&offset=1"));
        assert_eq!(query.limit, Some(3));
        assert_eq!(query.offset, Some(1));
    }

    #[test]
    fn openapi_stubs_are_callable() {
        list_admin_orders();
        patch_admin_order();
    }
}
