pub fn search_in_file(args: Vec<String>) -> Result<String, String> {
    let query = &args[1];
    let file_name = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_name);
    Ok("1".to_string())
}