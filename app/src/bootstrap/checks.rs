use crate::shared::checks::{ check_postgres, check_cassandra, check_rustfs, check_redis };

pub async fn run_checks() -> Result<(), Box<dyn std::error::Error>> {
    check_postgres().await?;
    println!("### Postgres connected successfully ###");
    check_cassandra().await?;
    println!("### Cassandra connected successfully ###");
    check_rustfs().await?;
    println!("### Rustfs connected successfully ###");
    check_redis().await?;
    println!("### Redis connected successfully ###");
    Ok(())
}

