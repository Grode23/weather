mod json_structs;

use structopt::StructOpt;
use crate::json_structs::Forecast;

/// My arguments for this application
#[derive(StructOpt, Debug)]
#[structopt(name = "weather")]
struct Opt {

	/// Name of the city
	#[structopt(short, long, parse(try_from_str), default_value = "MOSCOW")]
	city: String,

	/// State code of the location
	#[structopt(short, long, parse(from_str), default_value = "SKG")]
	state: String,

	/// Name of the country
	#[structopt(short = "l", long, parse(from_str), default_value = "Greece")]
	country: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let _opt = Opt::from_args();
	let answer = Forecast::get().await?;

	let (mins, maxs) = answer.get_temperatures();

	println!("Mins: {:?}", mins);
	println!("Maxs: {:?}", maxs);

	Ok(())
}
