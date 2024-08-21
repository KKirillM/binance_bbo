//! Looks like `cargo fmt` wasn't used, so I've done it.
//! Also, I've checked the project with `cargo clippy`, it found some notes.

use std::env;
use std::process;

mod config;
mod connector;
mod messages;
mod run;

const USAGE: &str = "Usage:\n\tbinance_bbo websocket_ip_addr:port currency_pair [currency_pair_2 ... currency_pair_n]\n
Example: binance_bbo wss://data-stream.binance.vision:9443 btcusdt ethusdt";

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = config::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        println!("{}", USAGE);
        process::exit(1);
    });

    //println!("IP-address: {}", config.ip_addr);
    //println!("Port: {}", config.port);
    //println!("Currencies: {:?}", config.currencies);

    if let Err(e) = run::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
