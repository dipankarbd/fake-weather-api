use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;

use crate::data::WeatherRepositoryState;

pub fn route_apis() -> Router<WeatherRepositoryState> {
    Router::new()
        .route("/current/:city", get(current_weather))
        .nest("/forecast", route_forecast())
}

fn route_forecast() -> Router<WeatherRepositoryState> {
    Router::new()
        .route("/fivedays/:city", get(forecast_five_days))
        .route("/twentyfourhours/:city", get(forecast_twenty_four_hours))
}

#[derive(Debug, Deserialize)]
struct PathParams {
    city: String,
}

async fn current_weather(
    State(repo): State<WeatherRepositoryState>,
    Path(params): Path<PathParams>,
) -> impl IntoResponse {
    let weather = repo.get_current_weather(&params.city).await;
    Json(weather)
}

async fn forecast_five_days(
    State(repo): State<WeatherRepositoryState>,
    Path(params): Path<PathParams>,
) -> impl IntoResponse {
    let forecast = repo.get_five_days_forecst(&params.city).await;
    Json(forecast)
}

async fn forecast_twenty_four_hours(
    State(repo): State<WeatherRepositoryState>,
    Path(params): Path<PathParams>,
) -> impl IntoResponse {
    let forecast = repo.get_twenty_four_hour_forecast(&params.city).await;
    Json(forecast)
}
