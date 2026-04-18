use config::{Config as ConfigCrate, ConfigError, Environment};  // Alias to avoid conflict
use serde::Deserialize;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::sync::{Arc, OnceLock};

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
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
    pub redis: Redis,
}

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
pub static DB_POOL: OnceLock<Arc<PgPool>> = OnceLock::new();

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

        Ok(())
    }

    pub fn get() -> &'static AppConfig {
        APP_CONFIG.get().expect("Config not initialized")
    }

    pub fn pool() -> &'static Arc<PgPool> {
        DB_POOL.get().expect("DB pool not initialized")
    }
}

