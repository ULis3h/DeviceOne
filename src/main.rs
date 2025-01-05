mod api;
mod config;
mod db;
mod error;
mod models;
mod services;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = config::Config::from_env()?;

    // Initialize database
    let db = db::Database::connect(&config.database_url).await?;

    // Setup API server
    let app = api::create_router(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
