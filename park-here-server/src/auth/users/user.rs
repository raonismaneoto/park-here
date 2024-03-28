use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub vehicles_ids: Vec<String>,
    pub id: String,
    pub name: String,
}
