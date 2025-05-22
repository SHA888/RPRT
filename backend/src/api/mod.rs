use axum::{
    routing::get,
    Router,
};

pub mod health;

pub fn create_router() -> Router {
    Router::new()
        .merge(health::create_health_router())
}
