use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;


use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::create(&args);
    println!("Searching for {}", config.query);
    println!("In the File : {}", config.filename);

    if let Err(e) = run(config) {
        println!("Unable Read the File! - {}", e);
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let mut f = File::open(config.filename)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;

    let results;

    if config.case_sensitivity {
        results = search_case_sensitive(&config.query, &content);
    } else {
        results = search(&config.query, &content)
    }

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_case_sensitive<'a>(query:&str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for lines in content.lines() {
        if lines.contains(&query){
            results.push(lines);
        }
    }
    results
}


pub fn search<'a>(query:&str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for lines in content.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()) {
            results.push(lines);
        }
    }
    results
}
