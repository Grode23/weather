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

pub fn get_accuracy_total(total_forecasts: i32) -> Vec<f32> {

    // The amount of temperatures
    // Minus one because the first date is the current date

    let mut rates: Vec<f32> = Vec::new();

    let standard_rate = 1.0 / total_forecasts as f32;

    let mut denominator = 1.0;
    for i in 0..total_forecasts {

        if i < total_forecasts / 2 {
            denominator = denominator * 2.0;
            rates.push(standard_rate + standard_rate/denominator);
        } else if i == total_forecasts / 2 && total_forecasts % 2 == 1 {
            rates.push(standard_rate);
        } else {
            rates.push(standard_rate - standard_rate/denominator);
            denominator = denominator / 2.0;
        }
    }

    rates
}












