use std::sync::Arc;

use axum::extract::{Path, Query, Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Json, Response};
use axum::RequestExt;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::parking::vacancies::vacancy::{VacancyStatus, VacancyType};
use crate::requests::payloads::{CreateVacancy, PatchVacancy};

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
    print!("received");
    let latitute = params.latitude;
    let longitude = params.longitude;
    let radius = params.radius;
    let t = params.vacancy_type.clone();
    let maybe_vacancies = app_state
        .vacancies_service
        .get_available_vacancies(latitute, longitude, VacancyType::from(t))
        .await;
    match maybe_vacancies {
        Ok(vacancies) => Json(vacancies).into_response(),
        Err(err) => Json(err.message()).into_response(),
    }
}

pub async fn auth_handler(State(app_state): State<Arc<AppState>>, mut req: Request) -> Request {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(v) => v,
        None => return req,
    };

    match app_state
        .auth_service
        .authorize(auth_header.to_string())
        .await
    {
        Ok(user) => {
            req.extensions_mut().insert(user);
            req
        }
        Err(err) => req,
    }
}
