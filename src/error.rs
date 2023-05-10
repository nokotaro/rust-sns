use actix_web::HttpResponse;
use eyre::Report;

pub struct Error;

impl Error {
    pub fn reqwest_error(err: Report) -> HttpResponse {
        if let Some(err) = err.downcast_ref::<reqwest::Error>() {
            if err.is_timeout() {
                HttpResponse::GatewayTimeout().body("")
            } else if err.is_connect() {
                HttpResponse::ServiceUnavailable().body("")
            } else if err.status().unwrap() == 404 {
                HttpResponse::NotFound().body("")
            } else {
                HttpResponse::InternalServerError().body("")
            }
        } else {
            HttpResponse::InternalServerError().body("")
        }
    }
}
