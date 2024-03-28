use serde::Serialize;

#[derive(Serialize)]
pub struct Car {
    pub plate: String,
    pub id: String,
}
