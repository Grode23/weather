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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

	let connection = establish_connection();

	// Get data from JSON
	let answer = Forecast::get().await?;

	// The prediction for the upcoming 5 days
	let temperatures = answer.get_temperatures();

	insert_temperature(&connection, temperatures);

	let date = Date::DateSaved;
	show_from_date(&connection, String::from("2020-09-10"), date);


	Ok(())
}