use std::{fs, error::Error, env};


pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;

    //println!("With text: {}", contents);

    println!("The results: ");

    if config.case_sensitive == true {
        for line in search_case_sensitive(&config.query, &contents){
            println!("{:?}",line);
        }
    }
    else{
        for line in search_case_insensitive(&config.query, &contents){
            println!("{:?}",line);
        }
    }

    Ok(())
}

pub struct Config{
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!")
        }

        let query: String = args[1].clone();
        let file_name: String = args[2].clone();
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, file_name, case_sensitive })
    }
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
                results.push(line.clone());
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(&query){
                results.push(line.clone());
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test(){
        let query = "duct";
        let contents = "\nRust:\nfast,safe,productive\nTrust me!";

        assert_eq!(vec!["fast,safe,productive"],search_case_insensitive(query, contents));
    }

    #[test]
    fn case_sensitive_search_test(){
        let query = "RuST";
        let contents = "\nRust:\nfast,safe,productive\nTrust me!";

        assert_eq!(vec!["Rust:","Trust me!"],search_case_sensitive(query, contents));
    }
}