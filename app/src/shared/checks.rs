use crate::shared::config::AppConfig;
use sqlx::query_as;
use tokio::time::{timeout, Duration};

// ========================================================================
// CASSANDRA 
// ========================================================================
pub async fn check_cassandra() -> Result<(), Box<dyn std::error::Error>> {
    let session = AppConfig::cassandra();

    timeout(
        Duration::from_secs(2),
        session.query_unpaged("SELECT release_version FROM system.local", &[])
    ).await??;
    Ok(())
}

// ========================================================================
// POSTGRES
// ========================================================================
pub async fn check_postgres() -> Result<(), Box<dyn std::error::Error>> {
    let pool = AppConfig::postgres();

    timeout(
        Duration::from_secs(2),
        query_as::<_,(i32,)>("SELECT 1").fetch_one(pool.as_ref())
    )
    .await??;
    Ok(())
}

// ========================================================================
// RUSTFS
// ========================================================================
pub async fn check_rustfs() -> Result<(), Box<dyn std::error::Error>> {
    let client = AppConfig::s3_client();

    timeout(
        Duration::from_secs(2),
        client.list_buckets().send()
    )
    .await??;
    Ok(())
}

// =========================================================================
// REDIS 
// =========================================================================
pub async fn check_redis() -> Result<(), Box<dyn std::error::Error>>{
    let redis_pool = AppConfig::redis();

    let mut conn = timeout(Duration::from_secs(2), redis_pool.get()).await??;
    
    let pong: String = timeout(
        Duration::from_secs(2),
        deadpool_redis::redis::cmd("PING").query_async(&mut conn),
    )
    .await??;

    if pong == "PONG" {
        Ok(())
    } else {
        Err("Unexpected PING response".into())
    }
}
