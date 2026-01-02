mod search;

use std::{env, fs};
use std::error::Error;
use std::process;
use minigrep::Config;
use crate::search::search_in_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args.as_slice()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}", config);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let res = search_in_file(config.query.as_str(), &contents);
    for line in res.iter() {
        println!("{}", line);
    }
    Ok(())
}