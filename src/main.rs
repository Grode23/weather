#[macro_use]
extern crate diesel;
extern crate dotenv;

mod accuweather_json;
mod database_stuff;
mod schema;
mod models;
mod calculations;
mod weatherbit_json;

// Uses of external libraries
use chrono::{Utc, DateTime};

// Uses of my own mods
use database_stuff::*;
use calculations::*;
use models::NewTemperature;
use accuweather_json::Forecast as Accuweather;
use weatherbit_json::Forecast as WeatherBit;
use std::io;

fn read() -> String{

	let mut answer = String::new();
	io::stdin()
		.read_line(&mut answer)
		.expect("Failed to read line");

	let length = answer.len();

	answer[0..length-1].to_string()
}

fn starting_menu() -> String {
	println!("1. API");
	println!("2. Database");
	println!("3. Calculations");
	println!("0. Exit");

	read()
}

fn api_menu(api: &str) -> String {
	println!("1. Change API [current: {}]", api);
	println!("2. Get data");

	read()
}

fn database_menu() -> String {
	println!("1. Insert current data");
	println!("2. Delete everything");
	println!("3. Insert dummy data");

	read()
}

fn calculations_menu(rate: &Rate, date: &String) -> String {
	println!("1. Change rate [current: {}]", rate);
	println!("2. Change date [current: {}]", date);
	println!("3. Calculate accuracy");

	read()
}

fn choose_api() -> String {

	println!("1. AccuWeather");
	println!("2. WeatherBit");
	println!("3. Dummy");

	let answer = read();

	match answer.as_str() {
		"1" => return String::from("accuweather"),
		"2" => return String::from("weatherbit"),
		"3" => return String::from("DUMMY"),
		_ => panic!("This is not a valid API"),
	}

}

async fn get_data(api: &String) -> Result<Vec<NewTemperature>, Box<dyn std::error::Error>> {

	// Get data from the API
	match api.as_str() {
		"accuweather" => {
			// Get data from JSON
			let answer = Accuweather::get().await?;
			// The prediction for the upcoming 5 days
			return Ok(answer.get_temperatures())
		},
		"weatherbit" => {
			// Get data from JSON
			let answer = WeatherBit::get().await?;
			// The prediction for the upcoming 5 days
			return Ok(answer.get_temperatures())
		},
		_ => {
			panic!("This API is not valid")
		},
	}

}

fn choose_rate() -> Rate {

	println!("1. Normal");
	println!("2. Simple");

	let answer = read();

	match answer.as_str() {
		"1" => return Rate::Normal,
		"2" => return Rate::Simple,
		_ => panic!("This is not a valid rate"),
	}
}

fn choose_date(date: &String) -> String {

	println!("Date format is: YYY-MM-DD or simply 'DUMMY'");

	let new_date = read();

	if (&new_date[4..5] != "-" || &new_date[7..8] != "-")
		&& new_date != String::from("DUMMY") {

		println!("The date format is not valid: {:?}", new_date.as_bytes());
		return String::from(date)
	}

	new_date
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

	let mut api: String = String::from("accuweather");
	let mut temperatures: Vec<NewTemperature> = Vec::new();
	let mut rate: Rate = Rate::Normal;

	// Get current date
	// e.g. `2014-11-28T12:45:59.324310806Z`
	let date: DateTime<Utc> = Utc::now();
	let mut date: String = date.to_string()[0..10].to_string();

	// Establish a connection with the database
	let connection = establish_connection();

	loop {
		let answer = starting_menu();
		let answer2: String;

		match answer.as_str() {
			// API
			"1" => {
				answer2 = api_menu(&api);
				match answer2.as_str() {
					// Change the API
					"1" => api = choose_api(),
					// Get data from the API
					"2" => temperatures = get_data(&api).await?,
					_ => println!("Incorrect answer to API menu"),
				}
			},
			// Database
			"2" => {
				answer2 = database_menu();
				match answer2.as_str() {
					// Insert
					"1" => insert_temperature(&connection, &temperatures),
					"2" => delete_all(&connection),
					"3" => add_dummy_data(&connection),
					_ => println!("Incorrect answer to Database menu"),
				}
			},
			// Calculation
			"3" => {
				answer2 = calculations_menu(&rate, &date);
				match answer2.as_str() {
					// Change rate
					"1" => rate = choose_rate(),
					// Get date
					"2" => date = choose_date(&date),
					// Calculate accuracy
					"3" => {
						let temperatures = get_from_date(&connection, &date);
						let accuracy = get_accuracy_total(&temperatures, Rate::Normal);

						println!("Accuracy of {} is {}", date, accuracy);
					}
					_ => println!("Incorrect answer to Calculation menu"),
				}
			},
			"0" => break,
			_ => println!("The answer was incorrect. Try again.{}", answer),
		}

	}

	Ok(())
}