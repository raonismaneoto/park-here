// src/main.rs

mod handlers;
pub mod parking;
pub mod regions;
pub mod auth;
pub mod database;
pub mod app_state;
pub mod AppError;

use std::{error::Error, net::SocketAddr};

use axum::{
    routing::get,
    routing::post,
    routing::patch,
    Router,
};

use crate::handlers::{get_available_vacancies_handler, create_vacancy, get_vacancy, search_vacancies, patch_vacancy};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

	let app = Router::new()
        .route("/api/park-here/health-check", get(|| async { "ParkHere's server is online" }))
        .route("/api/park-here/vacancies", get(get_available_vacancies_handler))
        .route("/api/park-here/vacancies/:vacancy_id", get(get_vacancy))
        .route("/api/park-here/vacancies", post(create_vacancy))
        .route("api/park-here/vacancies/:vacancy_id", patch(patch_vacancy))
        .route("api/park-here/vacancies/search", get(search_vacancies));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
