use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In the File : {}", filename);

    let content = fs::read_to_string(filename).expect("Unable to read the File!");

    let mut results= Vec::new();

    for lines in content.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()) {
            results.push(lines);
        }
    }

    for result in results {
        println!("{}", result);
    }

}
