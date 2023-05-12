use async_trait::async_trait;
use eyre::Result;

use crate::utils::DB_POOL;

#[derive(Debug, Clone, Copy)]
pub struct Driver {}

#[async_trait]
pub trait DriverTrait {
    async fn create_user(&self, name: String) -> Result<()>;
    async fn create_note(&self, user_id: u64, content: String) -> Result<()>;
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

    async fn create_note(&self, user_id: u64, content: String) -> Result<()> {
        let sql = &format!(r#"INSERT INTO notes (user_id, content) VALUES ($1, $2);"#);
        sqlx::query(&sql)
            .bind(user_id as f64)
            .bind(content)
            .execute(DB_POOL.get().unwrap())
            .await?;
        Ok(())
    }
}
