use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub city: String,
    pub current_temp: i32,
    pub max_temp: i32,
    pub min_temp: i32,
    pub condition: String,
    pub air_quality: i32,
    pub three_day_forecast: Vec<DailyForecast>,
}

#[derive(Debug, Serialize)]
pub struct FiveDayForecast {
    pub city: String,
    pub forecasts: Vec<DailyForecast>,
}

#[derive(Debug, Serialize)]
pub struct TwentyFourHourForecast {
    pub city: String,
    pub forecasts: Vec<HourlyForecast>,
}

#[derive(Debug, Serialize)]
pub struct HourlyForecast {
    pub temp: i32,
    pub hour: String,
    pub wind: f32,
    pub icon: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyForecast {
    pub day: String,
    pub date: String,
    pub condition: String,
    pub max_temp: i32,
    pub min_temp: i32,
    pub wind: f32,
    pub icon: String,
}
