use std::env;
use std::process;

use lifetimes::Config;
use lifetimes::run;

fn main() {
    // args() will give you an iterator over the command line arguments
    // .collect() gathers them into a vector
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}