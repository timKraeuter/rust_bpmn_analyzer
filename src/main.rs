use clap::Parser;
use rust_bpmn_analyzer::Config;
use std::process;

fn main() {
    let config = Config::parse();
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path);
    match collaboration {
        Ok(collaboration) => rust_bpmn_analyzer::run(&collaboration, config.properties),
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
