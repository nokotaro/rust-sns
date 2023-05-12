use derive_more::Constructor;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Notes(pub Vec<Note>);

#[derive(Debug, Eq, PartialEq, Clone, Constructor)]
pub struct Note {
    pub id: Option<u64>,
    pub user_id: u64,
    pub content: String,
}
