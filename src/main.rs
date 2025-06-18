use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Try to parse args into Config, or print error and exit
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Debug print the parsed values
    println!("Query: {:?}", config.query);
    println!("Filename: {:?}", config.filename);

    // Run the main logic, handle any errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
