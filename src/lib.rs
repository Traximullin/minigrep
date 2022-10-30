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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Не хватает аргрументов");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ingore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ingore_case            
        })
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

        let mut result = Vec::new();

        for line in contents.lines() { // разбивает текст на строки 
            if line.contains(query) { // проверяет содержит ли строка слово
                result.push(line);
            }  
        }

        result

    }

    pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
        
        let query = query.to_lowercase();
        let mut result = Vec::new();
        
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                result.push(line)
            }
        }

        result

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