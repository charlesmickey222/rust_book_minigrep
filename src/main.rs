use std::env;
use std::process;

use minigrep::Config;

fn main(){
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments:{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        println!("application error: {e}");
        process::exit(1);
    }
}

