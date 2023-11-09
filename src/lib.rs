mod bpmn;
use clap::Parser;

use crate::bpmn::{read_bpmn_file, GeneralProperty, ModelCheckingResult};
use std::error::Error;

/// CLI BPMN Analyzer written in Rust
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    /// File path to the BPMN file.
    #[arg(short, long, required = true)]
    pub file_path: String,

    /// BPMN properties to be checked
    #[arg(short, long, required = true, value_enum, value_delimiter = ',')]
    properties: Vec<GeneralProperty>,
}

pub fn run(config: Config) -> Result<ModelCheckingResult, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config.file_path);

    let start = collaboration.create_start_state();
    let result: ModelCheckingResult = collaboration.explore_state_space(start, config.properties);

    println!("Number of states: {}", result.state_space.states.len());
    println!("Property results: {:?}", result.property_results);

    Ok(result)
}
