use actix_web::{HttpResponse, Responder};
use tracing::info;

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
