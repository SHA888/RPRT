use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
    version: &'static str,
}

pub async fn health_check() -> impl IntoResponse {
    let response = HealthCheckResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION"),
    };
    (StatusCode::OK, axum::Json(response))
}

pub fn create_health_router() -> Router {
    Router::new().route("/health", get(health_check))
}
