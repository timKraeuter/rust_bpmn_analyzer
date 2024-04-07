use colored::Colorize;
use rust_bpmn_analyzer::model_checking::properties::ModelCheckingResult;
use std::time::Duration;

pub fn output_state_information(result: &ModelCheckingResult, runtime: Duration) {
    println!(
        "State space generation {} in {:?}!",
        "successful".green().bold(),
        runtime
    );
    println!(
        "States: {}, Transitions: {}",
        result.state_space.states.len(),
        result.state_space.count_transitions()
    );
    println!(
        "Terminated states: {}",
        result.state_space.terminated_state_hashes.len()
    );
}
