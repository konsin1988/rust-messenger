pub mod migrations;
pub mod checks;
pub mod buckets;

use std::error::Error;
use crate::bootstrap::{ migrations::run_migrations };
use crate::bootstrap::checks::{ run_checks };
use crate::bootstrap::buckets::create_buckets;

pub async fn run_startup_checks() -> Result<(), Box<dyn Error>> {
    run_checks().await?;
    run_migrations().await?;
    create_buckets().await?;
    Ok(())
}
