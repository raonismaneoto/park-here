use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ParkingVacancy {
    pub region: Region,
    pub id: String,
    pub status: VacancyStatus
}

#[derive(Serialize)]
pub struct Region {
    pub latitude: f32,
    pub longitude: f32
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub enum VacancyStatus {
    BUSY,
    FREE
}