use std::env;
use std::fs;

fn main() {
    // Grab the arguments 
    let args: Vec<String> = env::args().collect();
    
    // Storing information
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");


    // Read file 
    let content = fs::read_to_string(file_path).expect("ERROR: Wrong filepath given");

    println!("With content: {content}");



}
