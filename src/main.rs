use clap::Parser;
use log::error;

mod config;
mod run;
mod messages;
mod connector;

#[derive(Parser)]
#[command(author, version, about = "Binance BBO WebSocket client")]
struct Cli {
    /// WebSocket endpoint like wss://host:port
    url: String,
    /// Currency pairs to subscribe
    #[arg(required = true)]
    currencies: Vec<String>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let config = config::Config::new(&cli.url, &cli.currencies).unwrap_or_else(|err| {
        error!("{}", err);
        std::process::exit(1);
    });

    if let Err(e) = run::run(&config) {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}
