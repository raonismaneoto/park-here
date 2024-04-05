use std::sync::Arc;

use axum::extract::{Path, Query, Request, State};
use axum::http::header;
use axum::response::{IntoResponse, Json, Response};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::parking::vacancies::vacancy::VacancyType;
use crate::requests::payloads::{CreateVacancy, LoginPayload, PatchVacancy, SubscriptionPayload};

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
    if req.uri().path().contains("/api/park-here/login")
        || req.uri().path().contains("/api/park-here/subscribe")
    {
        return req;
    }

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

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Response {
    let login_result = app_state.auth_service.login(payload).await;

    match login_result {
        Ok(jwt) => Json(jwt).into_response(),
        Err(err) => Json(err.message()).into_response(),
    }
}

pub async fn subscribe_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<SubscriptionPayload>,
) -> Response {
    let sub_result = app_state.auth_service.subscribe(payload).await;

    match sub_result {
        Ok(user) => Json(user).into_response(),
        Err(err) => Json(err.message()).into_response(),
    }
}
