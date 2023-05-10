use crate::{domain::user::User, error::Error, usecase::user_usecase};

use super::Container;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserParams {
    pub name: String,
}

pub async fn create_user(
    data: web::Data<Container>,
    user: web::Json<CreateUserParams>,
) -> impl Responder {
    let user = User {
        id: None,
        name: user.name.clone(),
    };
    let status = user_usecase::create_user(data.user_port, user).await;

    match status {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("{:?}", err);
            Error::reqwest_error(err)
        }
    }
}
