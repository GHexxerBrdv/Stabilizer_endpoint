use axum::{routing::get, Router};
use crate::handlers::stabilizer;
use crate::routes::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/stabilizer/state", get(stabilizer::get_metrics))
}
