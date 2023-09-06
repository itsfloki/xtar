mod config;

use config::Config;
use std::error::Error;
use std::fs::File;
use std::{env, process};
use tar::Builder;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("[ERROR]: {}", e);
        process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.options.contains(&'c') {
        let file = File::create(&config.archive_file)?;
        let mut a = Builder::new(file);

        for path in &config.paths {
            a.append_path(path)?;
        }
    }

    Ok(())
}
