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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string")
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string")
        };



        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, file_name, case_sensitive })
    }
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
                results.push(line.clone());
        }
    }

    return results;
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
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