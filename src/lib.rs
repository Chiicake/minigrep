use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use crate::search::search_in_file;
mod search;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let res = search_in_file(config.query.as_str(), &contents);
    for line in res.iter() {
        println!("{}", line);
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "query: {}, filename: {}", self.query, self.filename)
    }
}

impl Config {
    pub fn new(args: &'_ [String]) ->  Result<Config, &'_ str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
                vec!["safe, fast, productive."],
                search::search_in_file(query, contents)
            );
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_in_file(query, contents)
        );
    }

}