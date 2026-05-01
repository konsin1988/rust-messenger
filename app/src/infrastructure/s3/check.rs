use crate::shared::config::AppConfig;
use tokio::time::{timeout, Duration};

pub async fn check_s3() -> Result<(), Box<dyn std::error::Error>> {
    let client = AppConfig::s3_client();

    timeout(
        Duration::from_secs(2),
        client.list_buckets().send()
    )
    .await??;
    Ok(())
}
