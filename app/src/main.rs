//use std::net::SocketAddr;
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::{trace::TraceLayer, normalize_path::NormalizePathLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use app::interfaces::http::routes::api_router;
use app::shared::config::AppConfig;
use app::infrastructure::bootstrap::run_startup_checks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
              .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    AppConfig::init().await?;  // Now async
    run_startup_checks().await?;

    // 2. Interfaces: Setup GraphQL Schema
    //let schema = interfaces::graphql::create_schema(pg_pool.clone());

    // 3. Orchestration: Start both servers
    //let grpc_task = tokio::spawn(infrastructure::server::run_grpc(scylla));
    //let gql_task = tokio::spawn(infrastructure::server::run_graphql(schema));

    //let _ = tokio::try_join!(grpc_task, gql_task);
    
    let app = api_router()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
