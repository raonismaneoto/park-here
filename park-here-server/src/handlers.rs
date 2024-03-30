use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Json, Response};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::parking::vacancies::vacancy::{VacancyStatus, VacancyType};

#[derive(Deserialize)]
pub struct CreateVacancy {
    latitude: f64,
    longitude: f64,
    id: String,
    status: VacancyStatus,
}

#[derive(Deserialize)]
pub struct PatchVacancy {
    latitude: Option<f64>,
    longitude: Option<f64>,
    status: Option<VacancyStatus>,
}
#[derive(Deserialize)]
pub struct SearchParams {
    latitude: f64,
    longitude: f64,
    radius: f64,
    vacancy_type: String,
}

pub async fn get_available_vacancies_handler(State(app_state): State<Arc<AppState>>) -> Response {
    Json("").into_response()
}

pub async fn get_vacancy(
    State(app_state): State<Arc<AppState>>,
    Path(vacancy_id): Path<String>,
) -> Response {
    Json("").into_response()
}

pub async fn create_vacancy(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateVacancy>,
) -> Response {
    Json("").into_response()
}

pub async fn patch_vacancy(
    State(app_state): State<Arc<AppState>>,
    Path(vacancy_id): Path<String>,
    Json(payload): Json<PatchVacancy>,
) -> Response {
    let a = match payload.latitude {
        Some(latitude) => "",
        None => "print!",
    };

    Json("").into_response()
}

pub async fn search_vacancies_handler(
    State(app_state): State<Arc<AppState>>,
    params: Query<SearchParams>,
) -> Response {
    let latitute = params.latitude;
    let longitude = params.longitude;
    let radius = params.radius;
    let t = params.vacancy_type.clone();
    let maybe_vacancies = app_state.vacancies_service.get_available_vacancies(
        latitute,
        longitude,
        VacancyType::from(t),
    ).await;
    match maybe_vacancies {
        Ok(vacancies) => Json(vacancies).into_response(),
        Err(err) => Json(err.message()).into_response(),
    }
}
