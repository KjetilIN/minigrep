use std::{error::Error, fs, env};

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if we have gotten 3 arguments at least
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Fetch environment variable for setting ignore case is true or not 
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path , ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read file 
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    }else{
        search(&config.query, &content)
    };

    for line in results{
        print!("{line}\n");
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

// Search function that is case insensitive
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();


    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }

    res

}

// Print command that outputs some basic information about the tool
pub fn print_commands(){
    print!("Minigrep - A Rust cli tool that implements the grep command on a small scale \n");
    print!("Usage with cargo: cargo run [WORD] [FILE.txt]\n");
    print!("    cargo run cm - List all commands available\n");
    print!("    cargo run [WORD] [FILENAME.txt] - List all lines that contains the given word (case insensitive)\n");
    print!("    IGNORE_CASE=false cargo run [WORD] [FILENAME.txt] - List all lines that contains the given word (case sensitive)\n");
}





// Tests for the search function 
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive!
What more could you want

        ";

        assert_eq!(vec!["safe, fast, productive!"], search(query, content))
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let content = "\
Rust:
safe, fast, productive!
What more could you want
        
         ";


        assert_eq!(vec!["Rust:"], search_case_insensitive(&query, &content))
    }
}