use crate::shared::config::AppConfig;
use crate::infrastructure::{
    cassandra::migrate::run_cassandra_migrations,
    postgres::migrate::run_postgres_migrations,
    s3::create::init_buckets,
};

pub async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let pool = AppConfig::postgres();
    run_postgres_migrations(pool).await?;

    let session = AppConfig::cassandra();
    run_cassandra_migrations(session).await?;

    let s3_client = AppConfig::s3_client();
    init_buckets(s3_client).await?;

    println!("### All migrations completed ###");
    Ok(())
}
