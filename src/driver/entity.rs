use derive_more::Constructor;

#[derive(Debug, Eq, PartialEq, Clone, Constructor, sqlx::FromRow)]
pub struct NoteEntity {
    pub id: i32,
    pub user_id: i64,
    pub content: String,
}
