use std::env;
use std::process::exit;

// Get the Config and run function from lib.rs
use minigrep::{Config, run, print_commands};

fn main() {
    // Grab the arguments 
    let args: Vec<String> = env::args().collect();

    // Command for listing commands
    if args[1] == "cm"{
        print_commands();
        exit(0);
    };
    
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




