use crate::models::Temperature;
use std::cmp;

// Get the rate of accuracy of a single date
pub fn get_accuracy_day(actual_temp: Temperature, forecast_temp: Temperature) -> f32 {

    // Values need to be casted into integers, otherwise I cannot compare them
    let actual_min = (actual_temp.minimum + 100.0 ) as i32;
    let actual_max = (actual_temp.maximum + 100.0 ) as i32;
    let forecast_min = (forecast_temp.minimum + 100.0 ) as i32;
    let forecast_max = (forecast_temp.maximum + 100.0 ) as i32;

    let min = cmp::min(actual_min, forecast_min);
    let max = cmp::max(actual_max, forecast_max);
    // The total range of the temperatures
    let range = max - min;

    let min = cmp::max(actual_min, forecast_min);
    let max = cmp::min(actual_max, forecast_max);
    // The range of accurate weather
    let accurate_range = max - min;

    // Rate of accuracy
    ( accurate_range as f32 / range as f32 )
}

pub fn get_accuracy_total(temperatures: &Vec<Temperature>) {

    // The amount of temperatures
    // Minus one because the first date is the current date
    let amount = temperatures.len() - 1 ;

    let mut rates: Vec<f32> = Vec::new();

    let standard_rate = 1.0 / amount as f32;

    let mut denominator = 1.0;
    for i in 0..amount / 2 {
        denominator = denominator * 2.0;
        rates.push(standard_rate + standard_rate/denominator);
    }

    let mut index = 0;
    if  amount % 2 != 0 {
        rates.push(standard_rate);
        index = 1;
    }

    for i in (amount / 2) .. amount - index{
        rates.push(standard_rate - standard_rate/denominator);
        denominator = denominator / 2.0;
    }

    for rate in rates {
        println!("{}", rate);
    }
}












