use crate::{domain::note::Note, driver::DriverTrait, port::NotePort};
use async_trait::async_trait;
use eyre::Result;

#[derive(Debug, Clone, Copy)]
pub struct NoteGateway<T: DriverTrait + Send + Sync> {
    pub driver: T,
}

#[async_trait]
impl<T: DriverTrait + Send + Sync> NotePort for NoteGateway<T> {
    async fn create_note(&self, note: Note) -> Result<()> {
        Ok(self.driver.create_note(note.user_id, note.content).await?)
    }
}
