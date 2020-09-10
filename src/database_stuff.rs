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

use super::models::NewTemperature;

pub fn insert_temperature(conn: &MysqlConnection, new_temperatures: Vec<NewTemperature>) {
    // Import the table
    use super::schema::temperatures;

    // let new_temperature = NewTemperature {
    //     minimum: 5.0,
    //     maximum: 10.6,
    //     date_of_temp: String::from("25"),
    //     date_today: String::from("24"),
    // };

    diesel::insert_into(temperatures::table)
        .values(&new_temperatures)
        .execute(conn)
        .expect("Error saving new post");
}
