use std::env;
use std::fs;

fn main() {
    // Grab the arguments 
    let args: Vec<String> = env::args().collect();
    
    // Storing information
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);


    // Read file 
    let content = fs::read_to_string(config.file_path).expect("ERROR: Wrong filepath given");

    println!("With content: {content}");



}


struct Config{
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // Check if we have gotten 3 arguments at least
        if args.len() < 3{
            panic!("Not enough arguments given")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config { query, file_path }
    }
}

