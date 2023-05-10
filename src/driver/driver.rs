use async_trait::async_trait;
use eyre::Result;

use crate::utils::DB_POOL;

#[derive(Debug, Clone, Copy)]
pub struct Driver {}

#[async_trait]
pub trait DriverTrait {
    async fn create_user(&self, name: String) -> Result<()>;
}

#[async_trait]
impl DriverTrait for Driver {
    async fn create_user(&self, name: String) -> Result<()> {
        let sql = &format!(r#"INSERT INTO users (name) VALUES ($1);"#);
        sqlx::query(&sql)
            .bind(name)
            .execute(DB_POOL.get().unwrap())
            .await?;
        Ok(())
    }
}
