use serde::Serialize;

// must be renamed to location
#[derive(Serialize)]
pub struct Region {
    pub latitude: f64,
    pub longitude: f64,
    pub id: String,
}
