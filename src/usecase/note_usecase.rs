use crate::{
    domain::note::{Note, Notes},
    port::note_port::NotePort,
};
use eyre::Result;

pub async fn create_note(note_port: impl NotePort, note: Note) -> Result<()> {
    let _note = note_port.create_note(note).await?;
    Ok(())
}

pub async fn get_timeline(note_port: impl NotePort) -> Result<Notes> {
    let notes = note_port.get_timeline().await?;
    Ok(notes)
}
