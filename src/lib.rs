use std::{error::Error, fs};

pub struct Config{
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if we have gotten 3 arguments at least
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read file 
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content){
        println!("{line}");
    }

    Ok(())

}


// Search function
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}




// Tests for the search function 
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive!
What more could you want

        ";

        assert_eq!(vec!["safe, fast, productive!"], search(query, content))
    }
}