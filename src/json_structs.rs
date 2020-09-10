#![allow(non_snake_case)]
use serde::Deserialize;
use reqwest::Url;
use crate::models::NewTemperature;

#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub Headline: Headline,
    DailyForecasts: Vec<DailyForecast>,
}

#[derive(Deserialize, Debug)]
pub struct Headline {
    EffectiveDate: String,
    EffectiveEpochDate: i32,
    Severity: i32,
    pub Text: String,
    Category: String,
    //EndDate: String,
    //EndEpochDate: i32,
    MobileLink: String,
    Link: String,
}

#[derive(Deserialize, Debug)]
struct DailyForecast {
    Date: String,
    EpochDate: i32,
    Temperature: Temperature,
    Day: DayNight,
    Night: DayNight,
    Sources: Vec<String>,
    MobileLink: String,
    Link: String,
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

#[derive(Deserialize, Debug)]
struct DayNight {
    Icon: i32,
    IconPhrase: String,
    HasPrecipitation: bool,
}

impl Forecast {

    pub async fn get() -> Result<Self, Box<dyn std::error::Error>>{

        let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{}?metric=true&apikey={}", 186405, "fQZ3PdGlJiJDVVOfRmEGAFyjGcrUArtH");
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
                date_of_temp: String::from(&daily_forecast.Date[0..10]),
                date_today: current_date.clone(),
            };

            temperatures.push(temperature);
        }

        temperatures
    }
}
