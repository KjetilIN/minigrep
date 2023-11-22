use std::env;
use std::process::exit;

// Get the Config and run function from lib.rs
use minigrep::{Config, run};

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




