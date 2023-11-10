mod bpmn;
mod model_checking;
mod states;
use clap::Parser;

use crate::bpmn::read_bpmn_file;
use crate::model_checking::bpmn_properties::{BPMNProperty, ModelCheckingResult};
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
    pub properties: Vec<BPMNProperty>,

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

    output_to_console(&result, runtime);

    Ok(result)
}

fn output_to_console(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}

fn output_property_results(result: &ModelCheckingResult) {
    for property_result in result.property_results.iter() {
        if property_result.fulfilled {
            println!("The property {:?} is fulfilled.", property_result.property)
        } else {
            print!(
                "The property {:?} is not fulfilled. ",
                property_result.property
            );
            match property_result.property {
                BPMNProperty::Safeness => {
                    println!(
                        "The sequence flow(s) {:?} can hold two or more tokens.",
                        property_result.problematic_elements
                    );
                }
                BPMNProperty::OptionToComplete => {
                    println!();
                }
                BPMNProperty::ProperCompletion => {
                    println!(
                        "The end event(s) {:?} consume two or more tokens.",
                        property_result.problematic_elements
                    );
                }
                BPMNProperty::NoDeadActivities => {
                    println!(
                        "The activities {:?} cannot be executed.",
                        property_result.problematic_elements
                    );
                }
            }
        }
    }
}

fn output_state_information(result: &ModelCheckingResult, runtime: Duration) {
    println!("State space generation successful in {:?}!", runtime);
    println!(
        "States: {}, Transitions: {}",
        result.state_space.states.len(),
        result.state_space.transitions.values().len()
    );
    println!(
        "Terminated states: {}",
        result.state_space.terminated_state_hashes.len()
    );
}
