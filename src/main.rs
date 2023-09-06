mod config;

use config::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    dbg!(config);
}
