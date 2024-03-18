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
#[derive(PartialEq)]
pub enum VacancyStatus {
    BUSY,
    FREE
}

impl From<VacancyStatus> for i32 {
    fn from(value: VacancyStatus) -> Self {
        if value == VacancyStatus::BUSY {
            0
        } else {
            1
        }
    }
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(PartialEq)]
pub enum VacancyType {
    CAR,
    MOTORCYCLE
}

impl From<VacancyType> for i32 {
    fn from(value: VacancyType) -> Self {
        if value == VacancyType::CAR {
            0
        } else {
            1
        }
    }
}