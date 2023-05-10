use actix_web::{HttpServer, App, web};

pub mod ping;

#[actix_web::main]
pub async fn build() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/v1/systems/ping").route(web::get().to(ping::ping)));
}