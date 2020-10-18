// Use it to be able to get the name of the table
// Chicka-chicka rhymes
use super::schema::{temperatures, accuracies, total};

pub enum Tables {
    Temperature,
    Accuracy,
}

#[derive(Queryable, Debug)]
pub struct Temperature {
    pub id: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub date_of_forecast: String,
    pub date_saved: String,
    pub api: String,
}

#[derive(Insertable, Debug)]
#[table_name="temperatures"]
pub struct NewTemperature {
    pub minimum: f32,
    pub maximum: f32,
    pub date_of_forecast: String,
    pub date_saved: String,
    pub api: String,
}

#[derive(Queryable, Debug)]
pub struct Accuracy {
    pub id: i32,
    pub accuracy: f32,
    pub date_of_forecast: String,
    pub api: String,
}

#[derive(Insertable, Debug)]
#[table_name="accuracies"]
pub struct NewAccuracy {
    pub accuracy: f32,
    pub date_of_forecast: String,
    pub api: String,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name="total"]
pub struct Total {
    pub api: String,
    pub accum_accuracy: f32,
}
