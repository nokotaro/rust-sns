use async_trait::async_trait;
use eyre::Result;

use crate::domain::note::Note;

#[async_trait]
pub trait NotePort {
    async fn create_note(&self, note: Note) -> Result<()>;
}
