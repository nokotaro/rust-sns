#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Note {
    pub id: Option<u64>,
    pub user_id: u64,
    pub content: String,
}
