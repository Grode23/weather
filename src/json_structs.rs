use serde::Deserialize;
use reqwest::Url;

#[derive(Deserialize, Debug)]
pub struct Forecast {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: MainInfo,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    pub sys: Sys,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct MainInfo {
    temp: f64,
    pressure: i32,
    humidity: i32,
    temp_min: f64,
    temp_max: f64,

}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    r#type: i32,
    id: i32,
    country: String,
    sunrise: u32,
    sunset: u32,
}

impl Forecast {

    pub async fn get(city: &str) -> Result<Self, Box<dyn std::error::Error>>{

        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, "a0ab7e38245efbba82cc3f2ec308fe2c");
        let url = Url::parse(&*url)?;

        let respone = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        Ok(respone)
    }
}
