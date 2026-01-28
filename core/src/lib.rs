mod bpmn;
pub mod model_checking;
pub mod states;

use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::reader;
use crate::bpmn::reader::UnsupportedBpmnElementsError;
pub use crate::model_checking::por::{
    AmpleSetConfig, AmpleSetStats, ModelCheckingResultWithStats, run_with_por,
};
pub use crate::model_checking::properties::{ModelCheckingResult, Property};

pub fn run(collaboration: &Collaboration, properties: Vec<Property>) -> ModelCheckingResult<'_> {
    collaboration.explore_state_space(properties)
}

pub fn read_bpmn_from_file(file_path: &str) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    reader::read_bpmn_from_file(file_path)
}

pub fn read_bpmn_from_string(
    contents: &str,
) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    reader::read_bpmn_from_string(contents) // Collaboration name is irrelevant atm.
}
