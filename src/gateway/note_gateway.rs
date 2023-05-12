use crate::{
    domain::note::{Note, Notes},
    driver::DriverTrait,
    port::NotePort,
};
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

    async fn get_timeline(&self) -> Result<Notes> {
        let note_entities = self.driver.get_timeline().await?;
        Ok(Notes(
            note_entities
                .into_iter()
                .map(|f| Note::new(Some(f.id as u64), f.user_id as u64, f.content))
                .collect::<Vec<_>>(),
        ))
    }
}
