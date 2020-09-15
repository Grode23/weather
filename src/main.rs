#[macro_use]
extern crate diesel;
extern crate dotenv;

mod json_structs;
mod database_stuff;
mod schema;
mod models;

use json_structs::Forecast;
use database_stuff::*;
use models::Date;
use structopt::StructOpt;

/// Arguments for program handling
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

	/// Add dummy data for testing
	#[structopt(short, long = "add-dummy")]
	dummy: bool,

	/// Get data from the API
	#[structopt(short = "a", long = "get-data-from-api")]
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

	// Insert dummy data
	if opt.dummy {
		add_dummy_data(&connection);
	}



	// let date = Date::DateSaved;
	// show_from_date(&connection, String::from("2020-09-10"), date);


	Ok(())
}