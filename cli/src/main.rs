mod output;

use crate::output::property_info::output_property_results;
use crate::output::state_space_info::output_state_information;
use clap::Parser;
use rust_bpmn_analyzer::{ModelCheckingResult, Property};
use std::process;
use std::time::{Duration, Instant};

/// CLI BPMN Analyzer written in Rust
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    /// File path to the BPMN file.
    #[arg(short, long, required = true)]
    pub file_path: String,

    /// BPMN properties to be checked.
    #[arg(short, long, value_enum, value_delimiter = ',')]
    pub properties: Vec<Property>,
}

fn main() {
    let config = Config::parse();
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&config.file_path);
    match collaboration {
        Ok(collaboration) => {
            let start_time = Instant::now();
            let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
            let runtime = start_time.elapsed();
            output_result(&result, runtime);
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}

fn output_result(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}
