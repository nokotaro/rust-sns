use async_trait::async_trait;
use eyre::Result;

use crate::utils::DB_POOL;

use super::NoteEntity;

#[derive(Debug, Clone, Copy)]
pub struct Driver {}

#[async_trait]
pub trait DriverTrait {
    async fn create_user(&self, name: String) -> Result<()>;
    async fn create_note(&self, user_id: u64, content: String) -> Result<()>;
    async fn get_timeline(&self) -> Result<Vec<NoteEntity>>;
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
            .bind(user_id as i64)
            .bind(content)
            .execute(DB_POOL.get().unwrap())
            .await?;
        Ok(())
    }

    async fn get_timeline(&self) -> Result<Vec<NoteEntity>> {
        let sql = &format!(r#"SELECT * FROM notes"#);
        Ok(sqlx::query_as::<_, NoteEntity>(&sql)
            .fetch_all(DB_POOL.get().unwrap())
            .await?)
    }
}
