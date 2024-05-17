use std::env;



pub struct Config {
    pub query:String,
    pub filename: String,
    pub case_sensitivity: bool
}

impl Config { 
    pub fn create(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Less Number of Arguements Provided!")
        }
        let case_sensitivity = env::var("CASE_SENSITIVITY").is_err();
        Config {
            query : args[1].clone(),
            filename : args[2].clone(),
            case_sensitivity : case_sensitivity
        }
    }
}