use crate::config::AppConfig;
use crate::db::migrate::run_migrations;

pub async fn check_migrations() -> Result<(), sqlx::Error> {
    let pool = AppConfig::pool();

    run_migrations(pool).await?;

    println!("### Postgres migrations completed ###");
    Ok(())
}
