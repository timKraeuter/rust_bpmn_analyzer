mod bpmn;
pub mod model_checking;
pub mod states;

use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::reader;
use crate::bpmn::reader::UnsupportedBpmnElementsError;
pub use crate::model_checking::properties::{
    ModelCheckingResult, ModelCheckingResultWithStats, Property,
};
pub use crate::states::ample_set::{AmpleSetConfig, AmpleSetStats};

pub fn run(collaboration: &Collaboration, properties: Vec<Property>) -> ModelCheckingResult<'_> {
    collaboration.explore_state_space(properties)
}

/// Run the model checker with partial order reduction enabled.
///
/// This function uses ample sets to reduce the state space while preserving
/// all properties. It returns additional statistics about the reduction achieved.
///
/// # Arguments
/// * `collaboration` - The BPMN collaboration to analyze
/// * `properties` - The properties to check
/// * `por_config` - Configuration for partial order reduction
///
/// # Returns
/// A `ModelCheckingResultWithStats` containing the results and reduction statistics
pub fn run_with_por(
    collaboration: &Collaboration,
    properties: Vec<Property>,
    por_config: AmpleSetConfig,
) -> ModelCheckingResultWithStats<'_> {
    collaboration.explore_state_space_with_por(properties, por_config)
}

pub fn read_bpmn_from_file(file_path: &str) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    reader::read_bpmn_from_file(file_path)
}

pub fn read_bpmn_from_string(
    contents: &str,
) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    reader::read_bpmn_from_string(contents) // Collaboration name is irrelevant atm.
}
