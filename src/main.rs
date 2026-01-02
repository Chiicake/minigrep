mod search;

use std::{env, fs};
use std::error::Error;
use std::process;
use minigrep::Config;
use minigrep::run;
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


