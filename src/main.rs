use clap::Parser;
use std::str::FromStr;

use moodycron::runner::App;
use moodycron::runner::Personality;

#[derive(Debug, Parser)]
struct Args {
    personality: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Args = Args::parse();
    let personality = Personality::from_str(&args.personality)?;

    App::new(personality).run().await;

    Ok(())
}
