use axum::response::{IntoResponse, Json, Response};

use crate::models::{ParkingVacancy};
use crate::services;

pub async fn get_available_vacancies() -> Response {
    Json(services::get_available_vacancies()).into_response()
}