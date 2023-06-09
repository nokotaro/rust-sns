use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

pub mod note;
pub mod ping;
pub mod user;

use crate::{
    driver::Driver,
    gateway::{NoteGateway, UserGateway},
    utils::{create_pool, DB_POOL, SETTINGS},
};

#[actix_web::main]
pub async fn build() -> std::io::Result<()> {
    let db_pool = create_pool().await;
    DB_POOL.set(db_pool).unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(SETTINGS.clone()))
            .app_data(Data::new(Container {
                user_port: UserGateway { driver: Driver {} },
                note_port: NoteGateway { driver: Driver {} },
            }))
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/v1/systems/ping").route(web::get().to(ping::ping)))
        .service(web::resource("/v1/user/create").route(web::post().to(user::create_user)))
        .service(web::resource("/v1/note/create").route(web::post().to(note::create_note)))
        .service(web::resource("/v1/note/timeline").route(web::get().to(note::get_timeline)));
}

pub struct Container {
    user_port: UserGateway<Driver>,
    note_port: NoteGateway<Driver>,
}
