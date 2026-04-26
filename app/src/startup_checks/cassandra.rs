use crate::config::AppConfig;
use std::path::Path;
use tokio::fs;

pub async fn check_cassandra() -> Result<(), Box<dyn std::error::Error>> {
    let session = AppConfig::cassandra();

    session
        .query_unpaged("SELECT release_version FROM system.local", &[])
        .await?;

    println!("### Cassandra connected successfully ###");
    Ok(())
}

pub async fn run_cassandra_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let session = AppConfig::cassandra();
    let path = Path::new("cassandra/migrations");

    let mut entries = fs::read_dir(path).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        files.push(entry.path());
    }
    files.sort();

    for file_path in files {
        if file_path.extension().and_then(|s| s.to_str()) != Some("cql") {
            continue;
        }

        let content = fs::read_to_string(&file_path).await?;

        println!("  ### Running migration: {:?}", file_path);

        for statement in content.split(';') {
            let stmt = statement.trim();
            if stmt.is_empty() {
                continue;
            }

            session.query_unpaged(stmt, &[]).await?;
        }
    }

    println!("### Cassandra migrations completed ###");
    Ok(())
}
