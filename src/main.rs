use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use reqwest::Url;

/// My arguments for this application
#[derive(StructOpt, Debug)]
#[structopt(name = "weather")]
struct Opt {

	/// Name of the city
	#[structopt(short, long, parse(try_from_str), default_value = "Thessaloniki")]
	city: String,

	/// State code of the location
	#[structopt(short, long, parse(from_str), default_value = "SKG")]
	state: String,

	/// Name of the country
	#[structopt(short = "l", long, parse(from_str), default_value = "Greece")]
	country: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
	coord: Coord,
	weather: Vec<Weather>,
	base: String,
	main: MainInfo,
	visibility: i32,
	wind: Wind,
	clouds: Clouds,
	dt: i32,
	sys: Sys,
	id: i32,
	name: String,
	cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
	lon: f64,
	lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
	id: i32,
	main: String,
	description: String,
	icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MainInfo {
	temp: f64,
	pressure: i32,
	humidity: i32,
	temp_min: f64,
	temp_max: f64,

}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
	speed: f64,
	deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
	all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
	r#type: i32,
	id: i32, 
	country: String,
	sunrise: u32,
	sunset: u32,
}

impl Forecast {

	async fn get(city: &str) -> Result<Self, Box<dyn std::error::Error>>{

		let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, "a0ab7e38245efbba82cc3f2ec308fe2c");
		let url = Url::parse(&*url)?;

		let respone = reqwest::get(url)
			.await?
			.json::<Forecast>()
			.await?;

		Ok(respone)
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();
	let answer = Forecast::get(&opt.city).await?;

	println!("{:?}", answer.sys);

	Ok(())
}
