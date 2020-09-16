#![allow(non_snake_case)]
use serde::Deserialize;
use reqwest::Url;
use std::env;

use super::models::NewTemperature;

#[derive(Deserialize, Debug)]
pub struct Forecast {
    DailyForecasts: Vec<DailyForecast>,
}

#[derive(Deserialize, Debug)]
struct DailyForecast {
    Date: String,
    Temperature: Temperature,
}

#[derive(Deserialize, Debug)]
struct Temperature {
    Minimum: MinMax,
    Maximum: MinMax,
}

#[derive(Deserialize, Debug)]
struct MinMax {
    Value: f32,
    Unit: String,
    UnitType: i32,
}

impl Forecast {

    pub async fn get() -> Result<Self, Box<dyn std::error::Error>>{

        let api_key = env::var("API_KEY").unwrap();

        let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{}?metric=true&apikey={}", 186405, api_key);
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        Ok(response)
    }

    pub fn get_temperatures(&self) -> Vec<NewTemperature>{
        let daily_forecasts = &self.DailyForecasts;

        let mut temperatures:Vec<NewTemperature> = Vec::new();

        let current_date = String::from(&daily_forecasts[0].Date[0..10]);
        for daily_forecast in daily_forecasts {

            let temperature = NewTemperature {
                minimum: daily_forecast.Temperature.Minimum.Value,
                maximum: daily_forecast.Temperature.Maximum.Value,
                date_of_forecast: String::from(&daily_forecast.Date[0..10]),
                date_saved: current_date.clone(),
            };

            temperatures.push(temperature);
        }

        temperatures
    }
}
