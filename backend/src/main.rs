mod api;
mod config;
mod error;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;

use crate::{
    config::Config,
    error::AppError,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    init_logging();
    
    // Load configuration
    let config = Config::from_env()?;
    info!("Starting server with config: {:#?}", config);

    // Configure CORS
    let allowed_origins = [
        "http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap(),
        "http://127.0.0.1:5173".parse::<axum::http::HeaderValue>().unwrap(),
    ];

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::list(allowed_origins))
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

    // Build our application with routes and middleware
    let app = api::create_router()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors)
        .with_state(());

    // Parse address
    let addr: SocketAddr = config.server.socket_addr();
    info!("Server listening on {}", addr);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server running on {}", addr);
    
    axum::serve(listener, app.into_make_service())
        .await?;

    Ok(())
}

fn init_logging() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rprt_backend=debug,tower_http=debug".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_ansi(true),
        )
        .init();
}
