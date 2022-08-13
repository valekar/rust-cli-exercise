use std::env;
use std::process;

use oracle::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        print!("Problem processing the args: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        print!("Application error : {} \n ", e);
        process::exit(1);
    }
}
