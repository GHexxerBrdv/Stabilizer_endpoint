pub mod health;
pub mod stabilizer;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::config::Config;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Config,
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(health::router())
        .merge(stabilizer::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
