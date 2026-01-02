use std::fs;

pub fn search_in_file<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let split = contents.split("\n").collect::<Vec<&str>>();
    let mut res = Vec::new();
    for line in split.iter() {
        if line.contains(query) {
            res.push(*line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().as_str().contains(query.to_lowercase().as_str()) {
            res.push(line);
        }
    }
    res
}