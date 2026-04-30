use axum::{Router, routing::get};
use crate::interfaces::http::handlers::health::{health_handler, ready_handler};

pub fn health_router() -> Router{
    Router::new()
        .route("/healthz", get(health_handler))
        .route("/readyz", get(ready_handler))
}

