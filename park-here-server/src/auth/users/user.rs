use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
}
