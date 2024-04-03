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

use axum::middleware;
use axum::{routing::get, routing::patch, routing::post, Router};

use crate::app_state::AppState;

use crate::handlers::{
    create_vacancy, get_available_vacancies_handler, get_vacancy, patch_vacancy,
    search_vacancies_handler, auth_handler
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Arc::new(AppState::build());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

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
        .route_layer(middleware::from_fn(auth_handler))
        .with_state(app_state);

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
