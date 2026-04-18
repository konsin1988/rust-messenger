pub mod health;

use axum::Router;
use health::health_router;

pub fn api_router() -> Router {
    Router::new()
        .nest("/api/v1", health_router())
}
