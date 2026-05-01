use crate::infrastructure::{
    postgres::check::check_postgres,
    cassandra::check::check_cassandra,
    cache::check::check_cache,
    s3::check::check_s3,
};

pub async fn run_checks() -> Result<(), Box<dyn std::error::Error>> {
    check_postgres().await?;
    println!("### Postgres connected successfully ###");
    check_cassandra().await?;
    println!("### Cassandra connected successfully ###");
    check_s3().await?;
    println!("### Rustfs connected successfully ###");
    check_cache().await?;
    println!("### Redis connected successfully ###");
    Ok(())
}

