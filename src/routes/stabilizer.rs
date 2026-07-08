use axum::{routing::get, Router};
use crate::handlers::stabilizer;
use crate::routes::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/stabilizer/metrics", get(stabilizer::get_metrics))
        .route("/api/v1/stabilizer/pools", get(stabilizer::list_pools))
        .route("/api/v1/stabilizer/pools/{address}", get(stabilizer::get_pool_by_address))
}
