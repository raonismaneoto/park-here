use serde::{Serialize};

#[derive(Serialize)]
pub struct Region {
    pub latitude: f32,
    pub longitude: f32
}