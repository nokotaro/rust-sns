use async_trait::async_trait;
use eyre::Result;

use crate::domain::user::User;

#[async_trait]
pub trait UserPort {
    async fn create_user(&self, user: User) -> Result<()>;
}
