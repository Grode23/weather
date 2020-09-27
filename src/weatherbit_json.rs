use serde::Deserialize;
use reqwest::Url;
use std::env;

use super::models::NewTemperature;

#[derive(Deserialize, Debug)]
pub struct Forecast {
    data: Vec<Data>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    valid_date: String,
    min_temp: f32,
    max_temp: f32,
}

impl Forecast {

    pub async fn get() -> Result<Self, Box<dyn std::error::Error>>{

        let api_key = env::var("WEATHERBIT_KEY").unwrap();

        let url = format!("https://api.weatherbit.io/v2.0/forecast/daily?city={}&key={}", "Thessaloniki", api_key);
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        Ok(response)
    }

    pub fn get_temperatures(&self) -> Vec<NewTemperature>{
        let forecasts = &self.data;

        let mut temperatures:Vec<NewTemperature> = Vec::new();

        let current_date = String::from(&forecasts.get(0).unwrap().valid_date);
        for forecast in forecasts {

            let temperature = NewTemperature {
                minimum: forecast.min_temp,
                maximum: forecast.max_temp,
                date_of_forecast: String::from(&forecast.valid_date),
                date_saved: current_date.clone(),
                api: String::from("weatherbit"),

            };

            temperatures.push(temperature);
        }

        temperatures
    }
}