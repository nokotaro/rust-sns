use async_trait::async_trait;
use eyre::Result;

#[async_trait]
pub trait UserPort {
    async fn create_user(&self) -> Result<()>;
}
