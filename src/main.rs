use std::{env, fs, process, error::Error};
struct Config{
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!")
        }

        let query: String = args[1].clone();
        let file_name: String = args[2].clone();
    
        Ok(Config { query, file_name })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let user_config = Config::new(&args).unwrap_or_else(|err | {
        println!("Problem while parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Seareching for : {}", user_config.query);
    println!("In file: {}", user_config.file_name);

    if let Err(e) = run(user_config){
        println!("Application Error: {}",e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;

    println!("With text: {}", contents);

    Ok(())
}
