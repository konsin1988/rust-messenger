use crate::shared::config::AppConfig;
use aws_sdk_s3::Client;

async fn create_bucket(bucket_name: &String, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let exists = client
        .head_bucket()
        .bucket(bucket_name)
        .send()
        .await
        .is_ok();
    if !exists {
        client.create_bucket().bucket(bucket_name).send().await?;
        println!("  ### Create bucket: {}", bucket_name);
    } else {
        println!("   ### Bucket exists: {}", bucket_name);
    }
    Ok(())
} 

pub async fn init_buckets(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::get();
    let buckets = &config.rustfs.buckets;

    for bucket in buckets {
        let _ = create_bucket(bucket, client).await?;
    }
    println!("### All buckets were created/are existing ###");
    Ok(())
}
