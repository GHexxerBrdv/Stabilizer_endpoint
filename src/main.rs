mod config;
mod db;
mod error;
mod handlers;
mod routes;

use anyhow::Context;
use config::Config;
use routes::{create_router, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting Stabilizer Axum server...");

    // 2. Load configuration from environment
    let config = Config::from_env().context("Failed to load environment configuration")?;
    tracing::info!("Configuration loaded. Host: {}, Port: {}", config.host, config.port);

    // 3. Initialize database pool
    tracing::info!("Initializing database connection pool...");
    let db_pool = db::init_db_pool(&config.database_url)
        .await
        .context("Failed to initialize database pool")?;
    tracing::info!("Database connection pool established successfully.");

    // 4. Build application router with state
    let state = AppState {
        db: db_pool,
        config: config.clone(),
    };
    let app = create_router(state);

    // 5. Bind to address and start serving
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(format!("Failed to bind to {}", addr))?;

    tracing::info!("Stabilizer API server listening on http://{}", addr);
    axum::serve(listener, app)
        .await
        .context("Axum server runner failed")?;

    Ok(())
}