use std::eprintln;

use crate::{
    domain::note::{Note, Notes},
    error::Error,
    usecase::note_usecase,
};

use super::Container;
use actix_web::{web, HttpResponse, Responder};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

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

pub async fn get_timeline(data: web::Data<Container>) -> impl Responder {
    let notes = note_usecase::get_timeline(data.note_port).await;

    match notes {
        Ok(notes) => HttpResponse::Ok().json(notes_to_json(notes)),
        Err(err) => {
            eprintln!("{:?}", err);
            Error::reqwest_error(err)
        }
    }
}
fn notes_to_json(notes: Notes) -> NotesJson {
    NotesJson::new(
        notes
            .0
            .into_iter()
            .map(|note| NoteJson::new(note.user_id, note.content))
            .collect::<Vec<_>>(),
    )
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct NotesJson {
    pub notes: Vec<NoteJson>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct NoteJson {
    pub user_id: u64,
    pub content: String,
}
