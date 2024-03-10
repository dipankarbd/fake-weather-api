use chrono::{Datelike, TimeDelta, Timelike, Utc};
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

use crate::model::{
    CurrentWeather, DailyForecast, FiveDayForecast, HourlyForecast, TwentyFourHourForecast,
};

pub struct WeatherRepository {}
pub type WeatherRepositoryState = Arc<WeatherRepository>;

const ICONS: [&str; 10] = [
    "sun.max",
    "sun.rain",
    "sun.haze",
    "sun.dust",
    "cloud",
    "cloud.drizzle",
    "cloud.rain",
    "cloud.sun",
    "wind.snow",
    "snowflake",
];

const CONDITIONS: [&str; 5] = ["Clear", "Haze", "Clody", "Rain", "Snow"];

impl WeatherRepository {
    pub fn new() -> Self {
        WeatherRepository {}
    }

    pub async fn get_current_weather(&self, city: &str) -> CurrentWeather {
        time::sleep(Duration::from_millis(100)).await;

        let mut rng = thread_rng();
        let mut forecasts: Vec<DailyForecast> = Vec::with_capacity(5);
        for i in 0..3 {
            let dt = Utc::now() + TimeDelta::try_days(i).unwrap();
            let day = dt.weekday();
            let date = format!("{}/{}", dt.month(), dt.day());

            let temp: i32 = rng.gen_range(0..47);

            let icon: &str = ICONS[rng.gen_range(0..10)];
            let condition: &str = CONDITIONS[rng.gen_range(0..5)];
            let forecast = DailyForecast {
                day: day.to_string(),
                date,
                condition: condition.to_owned(),
                max_temp: temp + rng.gen_range(0..10),
                min_temp: temp,
                wind: rng.gen_range(0.0..100.0),
                icon: icon.to_owned(),
            };

            forecasts.push(forecast);
        }

        let temp: i32 = rng.gen_range(0..47);
        let condition: &str = CONDITIONS[rng.gen_range(0..5)];

        CurrentWeather {
            city: city.to_owned(),
            current_temp: temp,
            max_temp: temp + rng.gen_range(0..10),
            min_temp: temp - rng.gen_range(0..10),
            condition: condition.to_owned(),
            air_quality: rng.gen_range(50..200),
            three_day_forecast: forecasts,
        }
    }

    pub async fn get_five_days_forecst(&self, city: &str) -> FiveDayForecast {
        time::sleep(Duration::from_millis(100)).await;

        let mut rng = thread_rng();
        let mut forecasts: Vec<DailyForecast> = Vec::with_capacity(5);
        for i in 0..5 {
            let dt = Utc::now() + TimeDelta::try_days(-1 + i).unwrap();
            let day = dt.weekday();
            let date = format!("{}/{}", dt.month(), dt.day());

            let temp: i32 = rng.gen_range(0..47);

            let icon: &str = ICONS[rng.gen_range(0..10)];
            let condition: &str = CONDITIONS[rng.gen_range(0..5)];

            let forecast = DailyForecast {
                day: day.to_string(),
                date,
                condition: condition.to_owned(),
                max_temp: temp + rng.gen_range(0..10),
                min_temp: temp,
                wind: rng.gen_range(0.0..100.0),
                icon: icon.to_owned(),
            };

            forecasts.push(forecast);
        }

        FiveDayForecast {
            city: city.to_owned(),
            forecasts,
        }
    }

    pub async fn get_twenty_four_hour_forecast(&self, city: &str) -> TwentyFourHourForecast {
        time::sleep(Duration::from_millis(500)).await;

        let dt = Utc::now();
        let hour = dt.hour();

        let mut rng = thread_rng();
        let mut forecasts: Vec<HourlyForecast> = Vec::with_capacity(24);
        for i in 0..24 {
            let icon: &str = ICONS[rng.gen_range(0..10)];
            let forecast = HourlyForecast {
                temp: rng.gen_range(0..47),
                hour: format!("{:02}:00", (hour + i) % 24).to_owned(),
                wind: rng.gen_range(0.0..100.0),
                icon: icon.to_owned(),
            };

            forecasts.push(forecast);
        }

        TwentyFourHourForecast {
            city: city.to_owned(),
            forecasts: forecasts,
        }
    }
}
