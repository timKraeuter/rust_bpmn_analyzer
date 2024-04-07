mod bpmn;
mod model_checking;
mod output;
mod states;

use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::reader;
use crate::bpmn::reader::UnsupportedBpmnElementsError;
pub use crate::model_checking::properties::{ModelCheckingResult, Property};
use crate::output::property_info::output_property_results;
use crate::output::state_space_info::output_state_information;
use std::time::{Duration, Instant};

pub fn run(collaboration: &Collaboration, properties: Vec<Property>) -> ModelCheckingResult {
    let start_time = Instant::now();
    let result: ModelCheckingResult = collaboration.explore_state_space(properties);
    let runtime = start_time.elapsed();

    output_result(&result, runtime);

    result
}

pub fn read_bpmn_file(file_path: &String) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    reader::read_bpmn_file(file_path)
}

fn output_result(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}
