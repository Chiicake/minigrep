use std::fmt::{Display, Formatter};

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

