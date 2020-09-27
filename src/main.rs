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
use structopt::StructOpt;
use chrono::{Utc, DateTime};

// Uses of my own mods
use database_stuff::*;
use calculations::*;
use models::NewTemperature;
use accuweather_json::Forecast as Accuweather;
use weatherbit_json::Forecast as WeatherBit;

/// Arguments for program handling
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

	/// Add dummy data for testing
	#[structopt(long = "dummies")]
	dummy: bool,

	/// Delete everything from the database
	#[structopt(short, long)]
	delete: bool,

	/// Insert today's data
	#[structopt(short, long)]
	insert: bool,

	/// Choose a specific API and get today's data from it
	#[structopt(long = "api", default_value = "accuweather")]
	api_get: String,

	/// Get accuracy
	#[structopt(short, long)]
	accuracy: bool,

	/// The date of the forecast that will be calculated
	#[structopt(long, default_value = "today")]
	date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

	// Get arguments
	let opt = Opt::from_args();

	// Establish a connection with the database
	let connection = establish_connection();

	let temperatures: Vec<NewTemperature>;

	// Get data from the API
	match opt.api_get.as_str() {
		"accuweather" => {
			// Get data from JSON
			let answer = Accuweather::get().await?;
			// The prediction for the upcoming 5 days
			temperatures = answer.get_temperatures();
		},
		"weatherbit" => {
			// Get data from JSON
			let answer = WeatherBit::get().await?;
			// The prediction for the upcoming 5 days
			temperatures = answer.get_temperatures();
		},
		_ => {
			panic!("This API is not valid")
		},
	}

	// Delete everything and then exit
	if opt.delete {
		delete_all(&connection);
		println!("Everything is deleted");
		return Ok(())
	}

	// Insert the temperatures in the database
	if opt.insert {
		insert_temperature(&connection, temperatures);
	}

	// Insert dummy data and do all the process with those
	if opt.dummy {
		add_dummy_data(&connection);

		// I NEED ERROR HANDLING IN CASE OF ZERO SIZE (NO DATA)
		let temperatures = get_from_date(&connection, String::from("DUMMY"));

		let accuracy = get_accuracy_total(&temperatures, Rate::Normal);
		println!("Accuracy of dummy is: {}", accuracy);

	}

	// Calculate accuracy
	if opt.accuracy {

		// Get the date that the user wants
		match opt.date.as_str() {
			"today" => {
				// Get current date
				// e.g. `2014-11-28T12:45:59.324310806Z`
				let curr_date: DateTime<Utc> = Utc::now();
				let curr_date: String = curr_date.to_string();

				let temperatures = get_from_date(&connection, curr_date.clone());
				let accuracy = get_accuracy_total(&temperatures, Rate::Normal);

				println!("Accuracy of {} is {}", curr_date, accuracy);
			}
			date => {
				let temperatures = get_from_date(&connection, String::from(date));
				let accuracy = get_accuracy_total(&temperatures, Rate::Normal);

				println!("Accuracy of {} is {}", date, accuracy);
			}
		}

	}

	Ok(())
}