//! Liveness probe.

use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// JSON body for `GET /healthz`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct HealthResponse {
    /// Liveness status.
    pub status: String,
}

/// `GET /healthz` handler.
#[utoipa::path(
    get,
    path = "/healthz",
    responses((status = 200, description = "Process is up", body = HealthResponse))
)]
#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_owned(),
    })
}
