use std::sync::Arc;

use axum::extract::{Path, Query, Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Json, Response};
use axum::Extension;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::users::user::User;
use crate::parking::vacancies::vacancy::VacancyType;
use crate::requests::payloads::{CreateVacancy, LoginPayload, PatchVacancy, SubscriptionPayload};
use crate::AppError::auth::{AuthError, AuthErrorStatusCode};
use crate::AppError::error::AppError;

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
    Extension(maybe_current_user): Extension<Result<User, AuthError>>,
) -> Response {
    match maybe_current_user {
        Ok(user) => print!("{}", user),
        Err(err) => return get_error_response(Box::new(err))
    }

    let latitute = params.latitude;
    let longitude = params.longitude;
    let radius = params.radius;
    let t = params.vacancy_type.clone();
    let maybe_vacancies = app_state
        .vacancies_service
        .get_available_vacancies(latitute, longitude, VacancyType::from(t))
        .await;
    match maybe_vacancies {
        Ok(vacancies) => (StatusCode::OK, Json(vacancies)).into_response(),
        Err(err) => get_error_response(err),
    }
}

pub async fn auth_handler(State(app_state): State<Arc<AppState>>, mut req: Request) -> Request {
    if req.uri().path().contains("/api/park-here/login")
        || req.uri().path().contains("/api/park-here/subscribe")
    {
        return req;
    }

    let maybe_auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match maybe_auth_header {
        Some(v) => v,
        None => {
            req.extensions_mut().insert(Err::<User, AuthError>(AuthError {
                message: Some(String::from("Missing authentication header")),
                status_code: AuthErrorStatusCode::UNAUTHORIZED
            }));
            return req
        }
    };

    match app_state
        .auth_service
        .authorize(auth_header.to_string())
        .await
    {
        Ok(user) => {
            req.extensions_mut().insert(Ok::<User, AuthError>(user));
            req
        }
        Err(err) => {
            req.extensions_mut().insert(Err::<User, AuthError>(AuthError {
                message: Some(String::from("Invalid authentication token")),
                status_code: AuthErrorStatusCode::UNAUTHORIZED
            }));
            req
        },
    }
}

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Response {
    let login_result = app_state.auth_service.login(payload).await;

    match login_result {
        Ok(jwt) => (StatusCode::OK,  Json(jwt)).into_response(),
        Err(err) =>  get_error_response(err)
    }
}

pub async fn subscribe_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<SubscriptionPayload>,
) -> Response {
    let sub_result = app_state.auth_service.subscribe(payload).await;

    match sub_result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => get_error_response(err),
    }
}

fn get_error_response(error: Box<dyn AppError>) -> Response {
    print!("{}", error.status_code());
    match StatusCode::from_u16(error.status_code() as u16) {
        Ok(status_code) => (status_code, Json(error.in_short())).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.in_short())).into_response()
    }
}

fn get_auth_extension(mut req: Request) -> Result<User, AuthError> {
    match req.extensions_mut().get::<User>() {
        Some(user) => Ok(user.clone()),
        None => {
            match req.extensions_mut().get::<AuthError>() {
                Some(err) => Err(err.clone()),
                None => Err(AuthError {
                    message: Some(String::from("Authentication error")),
                    status_code: AuthErrorStatusCode::UNAUTHORIZED
                })
            }
        }
    }
}
