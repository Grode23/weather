use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use super::models::{Temperature, NewTemperature};

pub fn insert_temperature(conn: &PgConnection) -> Temperature {
    // Import the table
    use super::schema::temperatures;

    let new_temperature = NewTemperature {
        minimum: 5.0,
        maximum: 10.6,
        date_of_temp: String::from("25"),
        date_today: String::from("24"),
    };

    diesel::insert_into(temperatures::table)
        .values(&new_temperature)
        .get_result(conn)
        .expect("Error saving new post")
}
