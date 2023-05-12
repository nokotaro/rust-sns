use crate::{domain::note::Note, error::Error, usecase::note_usecase};

use super::Container;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNoteParams {
    pub user_id: u64,
    pub content: String,
}

pub async fn create_note(
    data: web::Data<Container>,
    note: web::Json<CreateNoteParams>,
) -> impl Responder {
    let note = Note {
        id: None,
        user_id: note.user_id,
        content: note.content.clone(),
    };
    let status = note_usecase::create_note(data.note_port, note).await;

    match status {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("{:?}", err);
            Error::reqwest_error(err)
        }
    }
}
