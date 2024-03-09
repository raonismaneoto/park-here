// src/main.rs

pub mod models;
pub mod services;
mod handlers;

use std::{error::Error, net::SocketAddr};

use axum::{
    routing::get,
    Router,
};

use crate::handlers::{get_available_vacancies};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

	let app = Router::new()
        .route("/", get(|| async { "Hello, Axum" }))
        .route("/vacancies", get(get_available_vacancies));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
