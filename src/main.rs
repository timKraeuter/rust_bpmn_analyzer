use bpmnanalyzer::Config;
use clap::Parser;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = bpmnanalyzer::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
