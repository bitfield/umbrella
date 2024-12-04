use clap::Parser;
use umbrella::{Provider, WeatherStack};

#[derive(Parser)]
struct Args {
    #[arg(short, long, env = "WEATHERSTACK_API_KEY")]
    api_key: String,
    location: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let ws = WeatherStack::new(&args.api_key);
    let location = args.location.join(" ");
    let weather = ws.get_weather(&location)?;
    println!("{weather}");
    Ok(())
}
