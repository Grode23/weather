#![allow(non_snake_case)]
use serde::Deserialize;
use reqwest::Url;

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
    Value: f64,
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

        let url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{}?metric=true&apikey={}", 186405, "oDWbqGBn8dx63C0KyvvwCu0uma4GUWZS");
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        Ok(response)
    }
}
