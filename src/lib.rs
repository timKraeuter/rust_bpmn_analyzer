pub mod bpmn;
mod model_checking;
mod output;
mod states;

pub mod dtos;

use clap::Parser;

use crate::bpmn::collaboration::Collaboration;
pub use crate::model_checking::properties::{ModelCheckingResult, Property};
use crate::output::property_info::output_property_results;
use crate::output::state_space_info::output_state_information;
use std::time::{Duration, Instant};
use crate::bpmn::reader;
use crate::bpmn::reader::UnsupportedBpmnElementsError;

/// CLI BPMN Analyzer written in Rust
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    /// Port to serve the web app.
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

pub fn run(
    collaboration: &Collaboration,
    properties: Vec<Property>,
    output: bool,
) -> ModelCheckingResult {
    let start_time = Instant::now();
    let result: ModelCheckingResult = collaboration.explore_state_space(properties);
    let runtime = start_time.elapsed();

    if output {
        output_result(&result, runtime);
    }

    result
}

fn output_result(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}

pub fn read_bpmn_string(contents: &str) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    reader::read_bpmn_string(contents, "No name".to_string()) // Collaboration name is irrelevant atm.
}