use crate::shared::config::AppConfig;
use tokio::time::{timeout, Duration};

pub async fn check_cassandra() -> Result<(), Box<dyn std::error::Error>> {
    let session = AppConfig::cassandra();

    timeout(
        Duration::from_secs(2),
        session.query_unpaged("SELECT release_version FROM system.local", &[])
    ).await??;
    Ok(())
}
