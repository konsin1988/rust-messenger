use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::infrastructure::{
    postgres::check::check_postgres,
    cassandra::check::check_cassandra,
    cache::check::check_cache,
    s3::check::check_s3,
};
use tokio::join;

pub async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

pub async fn ready_handler() ->  impl IntoResponse {
    let (pg, cass, rustfs, redis) = join!(
        async { check_postgres().await.is_ok() },
        async { check_cassandra().await.is_ok() },
        async { check_s3().await.is_ok() },
        async { check_cache().await.is_ok() },
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

