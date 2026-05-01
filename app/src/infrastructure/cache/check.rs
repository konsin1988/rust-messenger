use crate::shared::config::AppConfig;
use tokio::time::{timeout, Duration};

pub async fn check_cache() -> Result<(), Box<dyn std::error::Error>>{
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
