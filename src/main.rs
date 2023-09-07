mod config;

use config::Config;
use std::{env, error::Error, fs::File, process};
use tar::{Archive, Builder};

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
    // write archive file
    if config.options.contains(&'c') {
        let file = File::create(&config.archive_file)?;
        let mut a = Builder::new(file);

        for path in &config.paths {
            a.append_path(path)?;
        }
        println!("Archive written...");
        return Ok(());
    }

    // extract archive file
    if config.options.contains(&'x') {
        let file = File::open(&config.archive_file)?;
        let mut ar = Archive::new(file);

        ar.unpack("./")?;
        println!("Archive unpacked...");
        return Ok(());
    }

    Ok(())
}
