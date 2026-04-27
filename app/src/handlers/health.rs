use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::checks::{ check_postgres, check_cassandra, check_rustfs, check_redis };
use tokio::join;

pub async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

pub async fn ready_handler() ->  impl IntoResponse {
    let (pg, cass, rustfs, redis) = join!(
        async { check_postgres().await.is_ok() },
        async { check_cassandra().await.is_ok() },
        async { check_rustfs().await.is_ok() },
        async { check_redis().await.is_ok() },
    );

    let all_ok = pg && cass && rustfs && redis;
    let status = if all_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    let body = json!({
        "status": if all_ok { "ok" } else { "not_ready" },
        "services": {
            "postgres": pg,
            "cassandra": cass,
            "rustfs": rustfs,
            "redis": redis,
        }
    });
    (status, Json(body))
}

