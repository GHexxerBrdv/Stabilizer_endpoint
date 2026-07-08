use axum::{routing::get, Router};
use crate::handlers::health;
use crate::routes::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check))
}
