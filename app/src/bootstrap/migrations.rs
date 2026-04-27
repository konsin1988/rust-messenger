use crate::config::AppConfig;
use crate::db::migrate::{ run_postgres_migrations, run_cassandra_migrations };

pub async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let pool = AppConfig::postgres();
    run_postgres_migrations(pool).await?;

    let session = AppConfig::cassandra();
    run_cassandra_migrations(session).await?;

    println!("### All migrations completed ###");
    Ok(())
}
