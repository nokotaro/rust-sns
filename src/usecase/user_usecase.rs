use crate::{domain::user::User, port::user_port::UserPort};
use eyre::Result;

pub async fn create_user(user_port: impl UserPort, user: User) -> Result<()> {
    let _user = user_port.create_user(user).await?;
    Ok(())
}
