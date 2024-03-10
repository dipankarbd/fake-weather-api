use axum::Router;
use data::{WeatherRepository, WeatherRepositoryState};
use routes::route_apis;
use std::{net::SocketAddr, sync::Arc};

mod data;
mod model;
mod routes;

#[tokio::main]
async fn main() {
    let repo: WeatherRepositoryState = Arc::new(WeatherRepository::new());
    let app = Router::new().nest("/api", route_apis()).with_state(repo);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
