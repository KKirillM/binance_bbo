//! Looks like `cargo fmt` wasn't used, so I've done it.
//! Also, I've checked the project with `cargo clippy`, it found some notes.

use std::env;
use std::process;

mod config;
mod connector;
mod messages;
mod run;

/// There are usuful crates for CLI app dev, e.g. `clap`.
const USAGE: &str = "Usage:\n\tbinance_bbo websocket_ip_addr:port currency_pair [currency_pair_2 ... currency_pair_n]\n
Example: binance_bbo wss://data-stream.binance.vision:9443 btcusdt ethusdt";

fn main() {
    // It's possible to pass `env::args()` without additional allocation.
    let args: Vec<String> = env::args().collect();
    // Commented code is a noise.

    // Typically it's better to wrap all the `main` code to a function returning `Result`, handling it in `main` in generic way.
    let config = config::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        println!("{}", USAGE);
        process::exit(1);
    });

    // A bit meaningless name of module (`run`).
    if let Err(e) = run::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
