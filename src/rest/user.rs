use crate::{error::Error, usecase::user_usecase};

use super::Container;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_user(data: web::Data<Container>) -> impl Responder {
    let status = user_usecase::create_user(data.user_port).await;

    match status {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("{:?}", err);
            Error::reqwest_error(err)
        }
    }
}
