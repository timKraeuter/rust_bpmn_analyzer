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
    pub property_results: Vec<GeneralPropertyResult>
}
#[derive(Debug)]
pub struct GeneralPropertyResult {
    pub property: GeneralProperty,
    pub fulfilled: bool,
    // DeadActivities: Dead activities
    // Safeness: Unsafe sequence flows
    // OptionToComplete: empty or where stuck?
    pub problematic_elements: Vec<String>,
    // Path, i.e., Vec of transitions
    // counter_example: Vec<Transition>

}