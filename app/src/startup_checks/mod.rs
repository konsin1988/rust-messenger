pub mod cassandra;
pub mod postgres;
pub mod migrations;

use std::error::Error;
use crate::startup_checks::{ postgres::check_postgres, cassandra::{ check_cassandra, run_cassandra_migrations }, migrations::check_migrations };

pub async fn run_startup_checks() -> Result<(), Box<dyn Error>> {
    check_postgres().await?;
    check_migrations().await?;
    check_cassandra().await?;
    run_cassandra_migrations().await?;
    Ok(())
}
