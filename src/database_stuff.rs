use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use super::models::{Temperature, NewTemperature};
// To access the table
use crate::schema::temperatures::dsl::*;
use crate::schema::temperatures::columns::{date_saved, date_of_forecast};
use crate::models::*;

pub fn insert_temperature(conn: &MysqlConnection, new_temperatures: Vec<NewTemperature>) {

    // Import the table
    use super::schema::temperatures;

    // check if the current date is inserted again. if it is, don't insert it again and whoop an error

    diesel::insert_into(temperatures::table)
        .values(&new_temperatures)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn show_from_date(connection: &MysqlConnection, date: String, type_of_date: Date){

    let results: Vec<Temperature>;

    match type_of_date {
        Date::DateSaved => results = temperatures
            .filter(date_saved.eq(date))
            .load::<Temperature>(connection)
            .expect("Error loading temperatures from saved date"),
        Date::DateOfForecast => results = temperatures
            .filter(date_of_forecast.eq(date))
            .load::<Temperature>(connection)
            .expect("Error loading temperatures from saved date"),
    }


    for result in results {
        println!("{:?}", result);
    }

}