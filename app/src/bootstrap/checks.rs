use crate::checks::{ check_postgres, check_cassandra };

pub async fn run_checks() -> Result<(), Box<dyn std::error::Error>> {
    check_postgres().await?;
    check_cassandra().await?;
    println!("### All checks completed ###");
    Ok(())
}
