use std::env;
use std::process;
use rusty_boyz::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("In file {}", config.file_path);

    if let Err(e) = rusty_boyz::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
