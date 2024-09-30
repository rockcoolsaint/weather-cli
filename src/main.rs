use clap::{Parser};
use reqwest;

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

fn main() {
    let args: Args = Args::parse();

    println!("{}", args.days);
}
