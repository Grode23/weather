#[derive(Queryable, Debug)]
pub struct Temperature {
    pub id: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub date_of_forecast: String,
    pub date_saved: String,
}

// Use it to be able to get the name of the table
// Chicka-chicka rhymes
use super::schema::temperatures;

#[derive(Insertable, Debug)]
#[table_name="temperatures"]
pub struct NewTemperature {
    pub minimum: f32,
    pub maximum: f32,
    pub date_of_forecast: String,
    pub date_saved: String,
}

pub enum Date {
    DateOfForecast,
    DateSaved,
}
