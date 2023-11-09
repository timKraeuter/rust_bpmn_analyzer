use bpmnanalyzer::Config;
use std::process;
use clap::Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = bpmnanalyzer::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
