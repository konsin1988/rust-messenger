use crate::config::AppConfig;
use sqlx::query_as;

pub async fn check_postgres() -> Result<(), sqlx::Error> {
    let pool = AppConfig::pool();

    let _: (i32,) = query_as("SELECT 1")
        .fetch_one(pool.as_ref())
        .await?;

    println!("### Postgres connected successfully ###");
    Ok(())
}
