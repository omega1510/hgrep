use hgrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    println!(
        "Searching for \"{}\" in file \"{}\".",
        config.query, config.filename
    );

    if let Err(e) = hgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
