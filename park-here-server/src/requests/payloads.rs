use serde::Deserialize;

use crate::parking::vacancies::vacancy::VacancyStatus;

#[derive(Deserialize)]
pub struct CreateVacancy {
    pub latitude: f64,
    pub longitude: f64,
    pub id: String,
    pub status: VacancyStatus,
}

#[derive(Deserialize)]
pub struct PatchVacancy {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub status: Option<VacancyStatus>,
}

#[derive(Deserialize)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub passwd: String,
    pub username: String,
}
