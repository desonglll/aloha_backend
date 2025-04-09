use actix_web::{HttpResponse, Responder};
use tracing::info;

/// Health check endpoint
///
/// # API Documentation
///
/// ## GET /api/health_check
///
/// Simple health check endpoint to verify the API is running.
///
/// ### Response
/// - 200 OK: API is healthy
#[utoipa::path(
    get,
    path = "/api/health_check",
    responses(
        (status = 200, description = "API is healthy")
    )
)]
pub async fn health_check() -> impl Responder {
    info!("Health check running");
    HttpResponse::Ok().finish()
}
