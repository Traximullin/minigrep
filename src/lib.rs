use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ingore_case {
        Config::search_case_insensitive(&config.query, &contents)
    } else {
        Config::search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ingore_case: bool,
}

impl Config {

    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ingore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ingore_case            
        })
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()

    }

    pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{

        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()

    }

}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
    
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], Config::search(query, contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            Config::search_case_insensitive(query, contents)
        )
    }

}