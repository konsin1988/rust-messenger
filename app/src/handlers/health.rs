use axum::Json;
use serde_json::json;

pub async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

pub async fn ready_handler() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

