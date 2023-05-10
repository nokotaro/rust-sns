use crate::{driver::DriverTrait, port::UserPort};
use async_trait::async_trait;
use eyre::Result;

#[derive(Debug, Clone, Copy)]
pub struct UserGateway<T: DriverTrait + Send + Sync> {
    pub driver: T,
}

#[async_trait]
impl<T: DriverTrait + Send + Sync> UserPort for UserGateway<T> {
    async fn create_user(&self) -> Result<()> {
        Ok(())
    }
}
