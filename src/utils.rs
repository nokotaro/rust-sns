use eyre::Result;
use once_cell::sync::{Lazy, OnceCell};
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_pool() -> PgPool {
    let db_url = &format!(
        "postgres://{}:{}@{}:{}/rustsns",
        SETTINGS.db_user, SETTINGS.db_pass, SETTINGS.db_host, SETTINGS.db_port,
    );

    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .unwrap()
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().unwrap());
pub static DB_POOL: OnceCell<PgPool> = OnceCell::new();

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db_user: String,
    pub db_pass: String,
    pub db_host: String,
    pub db_port: String,
}

impl Settings {
    fn new() -> Result<Self> {
        Ok(envy::from_env::<Settings>()?)
    }
}
