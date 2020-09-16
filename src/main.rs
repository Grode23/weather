#[macro_use]
extern crate diesel;
extern crate dotenv;

mod json_structs;
mod database_stuff;
mod schema;
mod models;
mod calculations;

use json_structs::Forecast;
use structopt::StructOpt;

use database_stuff::*;
use models::Date;
use calculations::*;

/// Arguments for program handling
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

	/// Add dummy data for testing
	#[structopt(long = "dummies")]
	dummy: bool,

	/// Add dummy data for testing
	#[structopt(short, long)]
	delete: bool,

	/// Get data from the API
	#[structopt(short = "a", long = "api")]
	get_from_api: bool,

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

	// Get arguments
	let opt = Opt::from_args();

	// Establish a connection with the database
	let connection = establish_connection();

	// Get data from the API
	if opt.get_from_api {

		// Get data from JSON
		let answer = Forecast::get().await?;

		// The prediction for the upcoming 5 days
		let temperatures = answer.get_temperatures();

		insert_temperature(&connection, temperatures);
	}

	if opt.delete {
		delete_all(&connection);
	}

	// Insert dummy data
	if opt.dummy {
		add_dummy_data(&connection);
	}

	let date = Date::DateOfForecast;
	// I NEED ERROR HANDLING IN CASE OF ZERO SIZE (NO DATA)
	let temperatures = get_from_date(&connection, String::from("DUMMY"), date);

	let accuracy = get_accuracy_total(&temperatures, Rate::Normal);
	println!("Accuracy of dummy is: {}", accuracy);
	Ok(())
}