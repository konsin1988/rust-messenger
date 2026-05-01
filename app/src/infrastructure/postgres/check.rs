use crate::shared::config::AppConfig;
use sqlx::query_as;
use tokio::time::{timeout, Duration};

pub async fn check_postgres() -> Result<(), Box<dyn std::error::Error>> {
    let pool = AppConfig::postgres();

    timeout(
        Duration::from_secs(2),
        query_as::<_,(i32,)>("SELECT 1").fetch_one(pool.as_ref())
    )
    .await??;
    Ok(())
}
