// src/main.rs
pub mod AppError;
pub mod app_state;
pub mod auth;
pub mod database;
mod handlers;
pub mod parking;
pub mod regions;
pub mod requests;

use std::sync::Arc;
use std::{error::Error, net::SocketAddr};

use axum::middleware::{self, map_request_with_state};
use axum::{routing::get, routing::patch, routing::post, Router};
use tokio::net::TcpListener;

use crate::app_state::AppState;

use crate::handlers::{
    auth_handler, create_vacancy, get_available_vacancies_handler, get_vacancy, patch_vacancy,
    search_vacancies_handler,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Arc::new(AppState::build());

    let app = Router::new()
        .route(
            "/api/park-here/health-check",
            get(|| async { "ParkHere's server is online" }),
        )
        .route(
            "/api/park-here/vacancies",
            get(get_available_vacancies_handler),
        )
        .route("/api/park-here/vacancies/:vacancy_id", get(get_vacancy))
        .route("/api/park-here/vacancies", post(create_vacancy))
        .route("/api/park-here/vacancies/:vacancy_id", patch(patch_vacancy))
        .route(
            "/api/park-here/vacancies/search",
            get(search_vacancies_handler),
        )
        .route_layer(map_request_with_state(app_state.clone(), auth_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
