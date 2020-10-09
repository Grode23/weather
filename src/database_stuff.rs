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

use super::models::*;
// Import columns, so I can select them when I get the data
use crate::schema::temperatures::columns::{date_saved, date_of_forecast as date_of_forecast_temp};
use crate::schema::accuracies::columns::{accuracy, date_of_forecast as date_of_forecast_acc};
// Import the table to insert
use super::schema::{temperatures, accuracies};

pub fn insert_accuracy(conn: &MysqlConnection, new_accuracy: NewAccuracy) {

    let date = String::from(&new_accuracy.date_of_forecast);

    // νεα συναρτηση αντι για nodatafordate, που θα επιστρεφει ενα option. αν ειναι none, κανω insert
    // αν ειναι Ok, κραταει το id που χρειαζομαι και με αυτο θα κανω update

    if let Some(id) = no_data_for_date(conn, date, Tables::Accuracy) {
        // Update
        diesel::update(accuracies::table.find(id))
            .set(accuracy.eq(accuracy))
            .execute(conn)
            .expect(&format!("Unable to find post with id: {}", id));
    } else {
        // Insert
        diesel::insert_into(accuracies::table)
            .values(new_accuracy)
            .execute(conn)
            .expect("Error saving new post");
    }

}


pub fn insert_temperature(conn: &MysqlConnection, new_temperatures: &Vec<NewTemperature>) {

    let date: String;

    if !new_temperatures.is_empty() {
        // Get date from an index that is going to be inserted
        // Every index has the same date_saved, so it doesn't matter which one I get
        date = String::from(&new_temperatures.get(0).unwrap().date_saved);
    } else {
        println!("There are no data to insert into the database");
        return
    }


    // check if the current date is inserted again. if it is, print it without inserting again
    if let None = no_data_for_date(conn, date, Tables::Temperature) {
        diesel::insert_into(temperatures::table)
            .values(new_temperatures)
            .execute(conn)
            .expect("Error saving new post");
    } else {
        println!("This date is already inserted");
    }
}

pub fn delete_all(connection: &MysqlConnection) {
    diesel::delete(temperatures::table)
        .execute(connection)
        .expect("Error deleting posts");
}

pub fn get_from_date(connection: &MysqlConnection, date: &String) -> Vec<Temperature>{

    let temperatures_vec: Vec<Temperature>;

    temperatures_vec = temperatures::table
        .filter(date_of_forecast_temp.eq(date))
        .load::<Temperature>(connection)
        .expect("Error loading temperatures from forecast's date");

    temperatures_vec
}

fn no_data_for_date(connection: &MysqlConnection, date: String, table_name: Tables) -> Option<i32> {

    match table_name {
        Tables::Temperature => {
            let results: Vec<Temperature> = temperatures::table
                .filter(date_saved.eq(date))
                .load::<Temperature>(connection)
                .expect("Error loading temperatures from saved date");

            if results.is_empty() {
                return None
            }

            return Some(results[0].id)
        },
        Tables::Accuracy => {
            let results: Vec<Accuracy>= accuracies::table
                .filter(date_of_forecast_acc.eq(date))
                .load::<Accuracy>(connection)
                .expect("Error loading accuracies from date");

            if results.is_empty() {
                return None
            }

            return Some(results[0].id)
        },
        _ => {}
    }

    None
}

// Insert dummy data to check the output
// In case the user doesn't want to wait for a week to get the actual data
//
// Every part of the dummy data has an invalid date_of_forecast
// Thus they shall not be confused with the real data
pub fn add_dummy_data(connection: &MysqlConnection) {

    let dummy_data: Vec<NewTemperature> = vec![
        NewTemperature {
            minimum: 25.3,
            maximum: 31.4,
            date_of_forecast: String::from("DUMMY"),
            date_saved: String::from("1980-1-1"),
            api: String::from("DUMMY")
        },
        NewTemperature {
            minimum: 24.3,
            maximum: 30.4,
            date_of_forecast: String::from("DUMMY"),
            date_saved: String::from("1980-1-2"),
            api: String::from("DUMMY")
        },
        NewTemperature {
            minimum: 25.7,
            maximum: 32.0,
            date_of_forecast: String::from("DUMMY"),
            date_saved: String::from("1980-1-3"),
            api: String::from("DUMMY")
        },
        NewTemperature {
            minimum: 24.6,
            maximum: 31.9,
            date_of_forecast: String::from("DUMMY"),
            date_saved: String::from("1980-1-4"),
            api: String::from("DUMMY")
        },
        NewTemperature {
            minimum: 25.0,
            maximum: 31.7,
            date_of_forecast: String::from("DUMMY"),
            date_saved: String::from("1980-1-5"),
            api: String::from("DUMMY")
        }
    ];

    // Dummy data (as everything else) will not be inserted twice, if it already is in the database
    insert_temperature(connection, &dummy_data);
}