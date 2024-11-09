use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        
        //check to make sure there are at least 2 arguments passed
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
       //set query and file path to the second and third index's in the args input.
        let query = args[1].clone();
        let file_path = args[2].clone();
       
        //set ignore case based on whether or not there is a third argument passed in the command
        //line
        //if not, use the env variable "IGNORE_CASE"
        let ignore_case = if args.len() > 3 && args[args.len() - 1] == "1" {
            true
        } else if args.len() > 3 && args[args.len() - 1] == "0" {
            false
        } else {
            env::var("IGNORE_CASE").is_ok()
        };



        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

pub fn get_tags<'a>(args: &[String]) {
    // -- do logic --
}
