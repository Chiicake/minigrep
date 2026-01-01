mod search;
mod lib;

use std::{env, fs};
use std::error::Error;
use std::process;
use lib::Config;
use crate::search::search_in_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args.as_slice()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Config: {:}", config);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
}