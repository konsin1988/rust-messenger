use sqlx::PgPool;

pub async fn run_postgres_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    //let pool = AppConfig::postgres();
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    println!("### Postgres migrations run successfully ###");
    Ok(())
}
