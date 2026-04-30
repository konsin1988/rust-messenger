use crate::shared::config::AppConfig;

pub async fn create_buckets() -> Result<(), Box<dyn std::error::Error>> {
    let buckets = vec![
        "messenger-media",
        "messenger-avatar",
        "messenger-temp",
    ];
    let client = AppConfig::s3_client();

    for bucket in buckets {
        let exists = client
            .head_bucket()
            .bucket(bucket)
            .send()
            .await
            .is_ok();
        if !exists {
            client.create_bucket().bucket(bucket).send().await?;
            println!("  ### Create bucket: {}", bucket);
        } else {
            println!("   ### Bucket exists: {}", bucket);
        }
    }
    println!("### All buckets were created/are existing ###");
    Ok(())
}
