use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use crate::routes::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolMetrics {
    pub pool_address: String,
    pub token_0: String,
    pub token_1: String,
    pub reserve_0: String,
    pub reserve_1: String,
    pub tvl_usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalMetrics {
    pub total_tvl_usd: f64,
    pub volume_24h_usd: f64,
    pub active_pools_count: u64,
}

/// Retrieve global metrics for the stabilizer protocol.
pub async fn get_metrics(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let metrics = GlobalMetrics {
        total_tvl_usd: 1250000.50,
        volume_24h_usd: 75000.20,
        active_pools_count: 3,
    };

    Ok((StatusCode::OK, Json(metrics)))
}

/// List all tracked liquidity pools.
pub async fn list_pools(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let pools = vec![
        PoolMetrics {
            pool_address: "0x1111111111111111111111111111111111111111".to_string(),
            token_0: "USDC".to_string(),
            token_1: "USDT".to_string(),
            reserve_0: "500000".to_string(),
            reserve_1: "500000".to_string(),
            tvl_usd: 1000000.00,
        },
        PoolMetrics {
            pool_address: "0x2222222222222222222222222222222222222222".to_string(),
            token_0: "DAI".to_string(),
            token_1: "USDC".to_string(),
            reserve_0: "125000".to_string(),
            reserve_1: "125000".to_string(),
            tvl_usd: 250000.00,
        },
    ];

    Ok((StatusCode::OK, Json(pools)))
}

/// Get detailed metrics for a specific pool by its contract address.
pub async fn get_pool_by_address(
    Path(address): Path<String>,
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(AppError::BadRequest("Invalid pool address format".to_string()));
    }

    if address == "0x1111111111111111111111111111111111111111" {
        let pool = PoolMetrics {
            pool_address: address,
            token_0: "USDC".to_string(),
            token_1: "USDT".to_string(),
            reserve_0: "500000".to_string(),
            reserve_1: "500000".to_string(),
            tvl_usd: 1000000.00,
        };
        Ok((StatusCode::OK, Json(pool)))
    } else {
        Err(AppError::NotFound(format!("Pool with address {} not found", address)))
    }
}
