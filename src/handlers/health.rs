use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::error::AppError;
use crate::routes::AppState;

/// Basic liveness probe to verify the server is running.
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

/// Readiness probe to verify backend connections (like the database) are healthy.
pub async fn readiness_check(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await?;

    Ok((StatusCode::OK, Json(json!({ "status": "ready" }))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
