use std::env;
use std::fs;
use std::process::exit;
use std::error::Error;

struct Config{
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if we have gotten 3 arguments at least
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Read file 
    let content = fs::read_to_string(config.file_path)?;

    println!("With content: {content}");

    Ok(())

}

fn main() {
    // Grab the arguments 
    let args: Vec<String> = env::args().collect();
    
    // Storing information
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("ERROR: not valid arguments : {err}");
        exit(1);
    });

    // Running the config
    if let Err(e) =  run(config){
        println!("ERROR: error with the provided arguments: {e}");
        exit(1);
    }

}




