mod bpmn;
mod model_checking;
mod output;
pub mod states;
use clap::Parser;

use crate::bpmn::reader::{read_bpmn_string, UnsupportedBpmnElementsError};
pub use crate::model_checking::properties::{ModelCheckingResult, Property};
use crate::output::property_info::output_property_results;
use crate::output::state_space_info::output_state_information;
use std::time::{Duration, Instant};

/// CLI BPMN Analyzer written in Rust
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    /// Port to serve the web app.
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

pub fn run(
    bpmn_file_content: &str,
    properties: Vec<Property>,
    output: bool,
) -> Result<ModelCheckingResult, UnsupportedBpmnElementsError> {
    let collaboration = read_bpmn_string(bpmn_file_content, "123".to_string())?;

    let start = collaboration.create_start_state();
    let start_time = Instant::now();
    let result: ModelCheckingResult = collaboration.explore_state_space(start, properties);
    let runtime = start_time.elapsed();

    if output {
        output_result(&result, runtime);
    }

    Ok(result)
}

fn output_result(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}
