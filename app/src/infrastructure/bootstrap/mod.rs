pub mod migrations;
pub mod checks;

use migrations::run_migrations;
use checks::run_checks;
use std::error::Error;

pub async fn run_startup_checks() -> Result<(), Box<dyn Error>> {
    run_checks().await?;
    run_migrations().await?;
    Ok(())
}
