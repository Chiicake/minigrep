mod search;

use std::env;
use crate::search::search_in_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tool: &str = &args[1];
    let res = match tool {
        "file" => {
            search_in_file(args).unwrap();
        },
        _ => panic!("invalid argument")
    };
}
