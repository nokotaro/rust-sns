#[derive(Debug, Eq, PartialEq, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub name: String,
}
