use serde::{Deserialize, Serialize};
use crate::regions::region::{Region};

#[derive(Serialize)]
pub struct ParkingVacancy {
    pub region: Region,
    pub id: String,
    pub status: VacancyStatus,
    pub t: VacancyType
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub enum VacancyStatus {
    BUSY,
    FREE
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub enum VacancyType {
    CAR,
    MOTORCYCLE
}