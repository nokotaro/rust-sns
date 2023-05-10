use crate::port::user_port::UserPort;
use eyre::Result;

pub async fn create_user(user_port: impl UserPort) -> Result<()> {
    let _user = user_port.create_user().await?;
    Ok(())
}
