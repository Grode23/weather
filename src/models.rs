#[derive(Queryable)]
pub struct Temperature {
    pub id: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub date_of_temp: String,
    pub date_today: String,
}

use super::schema::temperatures;

#[derive(Insertable, Debug)]
#[table_name="temperatures"]
pub struct NewTemperature {
    pub minimum:  f32,
    pub maximum:  f32,
    pub date_of_temp:   String,
    pub date_today:  String,
}