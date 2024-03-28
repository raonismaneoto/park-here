use serde::Serialize;

// must be renamed to location
#[derive(Serialize)]
pub struct Region {
    pub latitude: f32,
    pub longitude: f32,
    pub id: String,
}
