mod bpmn;
mod model_checking;
mod output;
mod states;
use clap::Parser;

use crate::bpmn::read_bpmn_file;
use crate::model_checking::bpmn_properties::{ModelCheckingResult, Property};
use crate::output::property_info::output_property_results;
use crate::output::state_space_info::output_state_information;
use std::error::Error;
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

    /// File path for results.
    #[arg(short, long, default_value = "./bpmn-analysis-results.json")]
    pub result_path: Option<String>,
}

pub fn run(config: Config) -> Result<ModelCheckingResult, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config.file_path);

    let start = collaboration.create_start_state();

    let start_time = Instant::now();
    let result: ModelCheckingResult = collaboration.explore_state_space(start, config.properties);
    let runtime = start_time.elapsed();

    output_result(&result, runtime);

    Ok(result)
}

fn output_result(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}
