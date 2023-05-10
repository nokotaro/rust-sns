use async_trait::async_trait;
use eyre::Result;

use crate::utils::DB_POOL;

#[derive(Debug, Clone, Copy)]
pub struct Driver {}

#[async_trait]
pub trait DriverTrait {
    async fn create_user(&self, table_name: &String) -> Result<()>;
}

#[async_trait]
impl DriverTrait for Driver {
    async fn create_user(&self, table_name: &String) -> Result<()> {
        let create_sql: &str = &format!("TODO{}", table_name);
        sqlx::query(create_sql)
            .execute(DB_POOL.get().unwrap())
            .await?;
        Ok(())
    }
}
