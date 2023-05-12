use crate::{domain::note::Note, port::note_port::NotePort};
use eyre::Result;

pub async fn create_note(note_port: impl NotePort, note: Note) -> Result<()> {
    let _note = note_port.create_note(note).await?;
    Ok(())
}
