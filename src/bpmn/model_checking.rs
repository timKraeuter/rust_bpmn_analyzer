use crate::bpmn::StateSpace;

#[derive(Debug)]
pub enum GeneralProperty {
    OptionToComplete,
    Safeness,
    DeadActivities
}
#[derive(Debug)]
pub struct ModelCheckingResult {
    pub state_space: StateSpace,
    pub properties_results: Vec<GeneralPropertyResult>
}
#[derive(Debug)]
pub struct GeneralPropertyResult {
    pub property: GeneralProperty,
    pub fulfilled: bool,
    // Path, i.e., Vec of transitions
    // counter_example: Vec<Transition>
}