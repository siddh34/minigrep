use std::{env, process};
use minigrep::Config;

fn main() {
    let user_config = Config::new(env::args()).unwrap_or_else(|err | {
        eprintln!("Problem while parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Seareching for : {}", user_config.query);
    println!("In file: {}", user_config.file_name);

    if let Err(e) = minigrep::run(user_config){
        println!("Application Error: {}",e);
        process::exit(1);
    }
}
