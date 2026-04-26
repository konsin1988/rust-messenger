use config::{Config as ConfigCrate, ConfigError, Environment};  // Alias to avoid conflict
use serde::Deserialize;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::sync::{Arc, OnceLock};
use scylla::client::session::{ Session };
use scylla::client::session_builder::SessionBuilder;

#[derive(Debug, Deserialize, Clone)] 
pub struct Database {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cassandra {
    pub contactpoint: String,
    pub port: u16,
    pub datacenter: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub ttl: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Keycloak {
    pub client_id: String,
    pub client_secret: String,
    pub realm: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]  
pub struct AppConfig {
    pub postgres: Database,
    pub cassandra: Cassandra,
    pub redis: Redis,
}

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
pub static DB_POOL: OnceLock<Arc<PgPool>> = OnceLock::new();
pub static CASSANDRA_SESSION: OnceLock<Arc<Session>> = OnceLock::new();

impl AppConfig {
    pub async fn init() -> Result<(), ConfigError> {
        let cfg_builder = ConfigCrate::builder()  // Uses aliased ConfigCrate
            .add_source(Environment::default().separator("_"))
            .build()?;

        let cfg: AppConfig = cfg_builder.try_deserialize()?;
        APP_CONFIG.set(cfg.clone()).map_err(|_| ConfigError::Message("APP_CONFIG set failed".into()))?;

        let url = format!(
            "postgres://{user}:{password}@{host}:{port}/{name}",
            user = cfg.postgres.username,
            password = cfg.postgres.password,
            host = cfg.postgres.host,
            port = cfg.postgres.port,
            name = cfg.postgres.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .map_err(|e| ConfigError::Message(e.to_string()))?;
        DB_POOL.set(Arc::new(pool)).map_err(|_| ConfigError::Message("DB_POOL set failed".into()))?;

        // cassandra
        init_cassandra(&cfg.cassandra)
            .await
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        Ok(())
    }

    pub fn get() -> &'static AppConfig {
        APP_CONFIG.get().expect("Config not initialized")
    }

    pub fn pool() -> &'static Arc<PgPool> {
        DB_POOL.get().expect("DB pool not initialized")
    }

    pub fn cassandra() -> &'static Arc<Session> {
        CASSANDRA_SESSION
            .get()
            .expect("Cassandra session not initialized")
    }
}

pub async fn init_cassandra(cfg: &Cassandra) -> Result<(), Box<dyn std::error::Error>> {
    let contact_point = format!(
        "{}:{}",
        cfg.contactpoint,
        cfg.port
    );

    let session = SessionBuilder::new()
        .known_node(contact_point)
        .build()
        .await?;

    CASSANDRA_SESSION
        .set(Arc::new(session))
        .map_err(|_| "Cassandra already initialized")?;

    Ok(())
}

