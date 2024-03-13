use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Json, Response};
use serde::Deserialize;

use crate::models::{ParkingVacancy, VacancyStatus};
use crate::services;

pub async fn get_available_vacancies() -> Response {
    Json(services::get_available_vacancies()).into_response()
}

pub async fn get_vacancy(Path(vacancy_id): Path<String>) -> Response {
    Json("").into_response()
}

#[derive(Deserialize)]
pub struct CreateVacancy {
    latitude: f32,
    longitude: f32,
    id: String,
    status: VacancyStatus
}

pub async fn create_vacancy(Json(payload): Json<CreateVacancy>) -> Response {
    Json("").into_response()
}

#[derive(Deserialize)]
pub struct PatchVacancy {
    latitude: Option<f32>,
    longitude: Option<f32>,
    status: Option<VacancyStatus>
}

pub async fn patch_vacancy(Path(vacancy_id): Path<String>, Json(payload): Json<PatchVacancy>) -> Response {
    let a  = match payload.latitude {
        Some(latitude) => "",
        None => "print!"
    };

    Json("").into_response()
}

#[derive(Deserialize)]
pub struct SearchParams {
    latitute: f32,
    longitude: f32,
    radius: i32
}

pub async fn search_vacancies(params: Query<SearchParams>) -> Response {
    let latitute = params.latitute;
    let longitude = params.longitude;
    let radius = params.radius;
    Json("").into_response()
}
