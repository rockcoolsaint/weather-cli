use clap::{Parser};
use reqwest;
use dotenv;
use serde::Deserialize;

const LAT: f32 = -41.2;
const LON: f32 = 174.7;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    /// Number of days for the forcast
    #[arg(short, default_value_t = 0)]
    days: u8,
}

#[derive(Deserialize, Debug)]
struct Coord {
    lat: f32,
    lon: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct CurrentWeatherMain {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    coord: Coord,
    weather: Vec<Weather>,
    // base: CurrentWeatherMain
    main: CurrentWeatherMain
}

fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let mut api_key: Option<String> = None;
    for (key, value) in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
    }
    if api_key.is_none() {
        panic!("need API key");
    }
    let api_key = api_key.unwrap();

    let args: Args = Args::parse();
    
    let method = match args.days {
        0 => "weather",
        _ => "forecasting"
    };

    let cnt = args.days * 8;

    let url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={cnt}");

    let weather: CurrentWeather = reqwest::blocking::get(url)?
    .json()?;

    println!("{} {:?}",weather.main.temp, weather.weather[0].description);

    Ok(())
}
