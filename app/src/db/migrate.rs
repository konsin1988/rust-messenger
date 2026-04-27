use sqlx::PgPool;
use scylla::client::session::Session;
use std::path::Path;
use tokio::fs;

pub async fn run_postgres_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    println!("### Postgres migrations run successfully ###");
    Ok(())
}

pub async fn run_cassandra_migrations(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
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

    println!("### Cassandra migrations run successfully ###");
    Ok(())
}
