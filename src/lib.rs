use std::fmt::{Display, Formatter};
mod search;
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
}