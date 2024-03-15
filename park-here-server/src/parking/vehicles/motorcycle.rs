use serde::Serialize;

#[derive(Serialize)]
pub struct Motorcycle {
    pub plate: String,
    pub id: String
}