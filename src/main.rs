#[macro_use]
extern crate diesel;
extern crate dotenv;

mod json_structs;
mod database_stuff;
mod schema;
mod models;

use json_structs::Forecast;
use database_stuff::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

	let answer = Forecast::get().await?;

	let temperatures = answer.get_temperatures();

	for temp in &temperatures {
		println!("{:?}", temp);
	}

	let connection = establish_connection();

	insert_temperature(&connection, temperatures);

	Ok(())
}
