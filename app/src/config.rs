use config::{Config as ConfigCrate, ConfigError, Environment};  // Alias to avoid conflict
use serde::Deserialize;
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::sync::{Arc, OnceLock};
use scylla::client::session::{ Session };
use scylla::client::session_builder::SessionBuilder;
use aws_sdk_s3::Client;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::config::Credentials;
use deadpool_redis::{Config, Runtime, Pool as RedisPool };


#[derive(Debug, Deserialize, Clone)] 
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cassandra {
    pub contact_point: String, 
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
pub struct Rustfs {
    pub endpoint_url: String,
    pub access_key: String,
    pub secret_key: String,
    pub data_dir: String,
}

#[derive(Debug, Deserialize, Clone)]  
pub struct AppConfig {
    pub postgres: Postgres,
    pub cassandra: Cassandra,
    pub redis: Redis,
    pub rustfs: Rustfs,
}

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
pub static DB_POOL: OnceLock<Arc<PgPool>> = OnceLock::new();
pub static CASSANDRA_SESSION: OnceLock<Arc<Session>> = OnceLock::new();
pub static S3_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
pub static CACHE: OnceLock<RedisPool> = OnceLock::new();

impl AppConfig {
    pub async fn init() -> Result<(), ConfigError> {
        let cfg_builder = ConfigCrate::builder()  // Uses aliased ConfigCrate
            .add_source(Environment::default().separator("__"))
            .build()?;

        let cfg: AppConfig = cfg_builder.try_deserialize()?;
        APP_CONFIG.set(cfg.clone()).map_err(|_| ConfigError::Message("APP_CONFIG set failed".into()))?;

        // postgres
        init_postgres(&cfg.postgres)
            .await
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        // cassandra
        init_cassandra(&cfg.cassandra)
            .await
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        // rustfs
        init_s3_client(&cfg.rustfs)
            .await
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        init_redis_pool(&cfg.redis)
            .await
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        Ok(())
    }

    pub fn get() -> &'static AppConfig {
        APP_CONFIG.get().expect("Config not initialized")
    }

    pub fn postgres() -> &'static Arc<PgPool> {
        DB_POOL.get().expect("DB pool not initialized")
    }

    pub fn cassandra() -> &'static Arc<Session> {
        CASSANDRA_SESSION
            .get()
            .expect("Cassandra session not initialized")
    }

    pub fn s3_client() -> &'static Arc<Client> {
        S3_CLIENT
            .get()
            .expect("S3 Client not initialized")
    }

    pub fn redis() -> &'static RedisPool {
        CACHE
            .get()
            .expect("Redis pool not initialized")
    }

}

pub async fn init_cassandra(cfg: &Cassandra) -> Result<(), Box<dyn std::error::Error>> {
    let contact_point = format!(
        "{}:{}",
        cfg.contact_point,
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

pub async fn init_postgres(postgres: &Postgres) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "postgres://{user}:{password}@{host}:{port}/{name}",
        user = postgres.username,
        password = postgres.password,
        host = postgres.host,
        port = postgres.port,
        name = postgres.database
    );

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .map_err(|e| ConfigError::Message(e.to_string()))?;
    DB_POOL.set(Arc::new(pool)).map_err(|_| ConfigError::Message("DB_POOL set failed".into()))?;

    Ok(())
}

pub async fn init_s3_client(cfg: &Rustfs) -> Result<(), Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));

    let credentials = Credentials::new(
        cfg.access_key.clone(),
        cfg.secret_key.clone(),
        None, None, 
        "rustfs",
    );
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .credentials_provider(credentials)
            .endpoint_url(cfg.endpoint_url.clone()) 
            .load()
            .await;
    
    let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();
    let client = Client::from_conf(s3_config);

    S3_CLIENT
        .set(Arc::new(client))
        .map_err(|_| ConfigError::Message("S3 client already initialized".into()))?;

    Ok(())
}

pub async fn init_redis_pool(redis: &Redis) -> Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "redis://:{password}@{host}:{port}/",
        password = redis.password,
        host = redis.host,
        port = redis.port,
    );
    let cfg = Config::from_url(url);

    let redis_pool = cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");
    CACHE.set(redis_pool)
        .expect("Redis pool already initialized");

    Ok(())
}

