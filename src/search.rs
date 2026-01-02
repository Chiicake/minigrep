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