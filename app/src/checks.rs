use crate::config::AppConfig;
use sqlx::query_as;

// ========================================================================
// CASSANDRA 
// ========================================================================
pub async fn check_cassandra() -> Result<(), Box<dyn std::error::Error>> {
    let session = AppConfig::cassandra();

    session
        .query_unpaged("SELECT release_version FROM system.local", &[])
        .await?;

    println!("### Cassandra connected successfully ###");
    Ok(())
}

// ========================================================================
// POSTGRES
// ========================================================================
pub async fn check_postgres() -> Result<(), sqlx::Error> {
    let pool = AppConfig::postgres();

    let _: (i32,) = query_as("SELECT 1")
        .fetch_one(pool.as_ref())
        .await?;

    println!("### Postgres connected successfully ###");
    Ok(())
}
