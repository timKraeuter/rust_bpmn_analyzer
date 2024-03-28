use bpmnanalyzer::Config;
use clap::Parser;
use std::process;

fn main() {
    let config = Config::parse();
    let collaboration = bpmnanalyzer::read_bpmn_file(&config.file_path);
    match collaboration {
        Ok(collaboration) => bpmnanalyzer::run(&collaboration, config.properties),
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
