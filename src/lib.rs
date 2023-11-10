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

    /// File path to export the generated state space.
    #[arg(short, long)]
    pub state_space_path: Option<String>,
}

pub fn run(config: Config) -> Result<ModelCheckingResult, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config.file_path);

    let start = collaboration.create_start_state();
    let result: ModelCheckingResult = collaboration.explore_state_space(start, config.properties);

    output_to_console(&result);
    // TODO: Make it possible to export the state space to a file by a param (json).

    Ok(result)
}

fn output_to_console(result: &ModelCheckingResult) {
    output_state_information(result);
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
                GeneralProperty::Safeness => {
                    println!(
                        "The sequence flow(s) {:?} can hold two or more tokens.",
                        property_result.problematic_elements
                    );
                }
                GeneralProperty::OptionToComplete => {
                    println!();
                }
                GeneralProperty::ProperCompletion => {
                    println!(
                        "The end event(s) {:?} consume two or more tokens.",
                        property_result.problematic_elements
                    );
                }
                GeneralProperty::NoDeadActivities => {
                    println!(
                        "The activities {:?} cannot be executed.",
                        property_result.problematic_elements
                    );
                }
            }
        }
    }
}

fn output_state_information(result: &ModelCheckingResult) {
    println!("State space generation successful!");
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
