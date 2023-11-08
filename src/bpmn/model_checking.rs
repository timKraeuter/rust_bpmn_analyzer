use crate::bpmn::StateSpace;

#[derive(Debug, PartialEq)]
pub enum GeneralProperty {
    OptionToComplete,
    Safeness,
    NoDeadActivities,
}

#[derive(Debug)]
pub struct ModelCheckingResult {
    pub state_space: StateSpace,
    pub property_results: Vec<GeneralPropertyResult>,
}
#[derive(Debug, PartialEq)]
pub struct GeneralPropertyResult {
    pub property: GeneralProperty,
    pub fulfilled: bool,
    // DeadActivities: Dead activities
    // Safeness: Unsafe sequence flows
    // OptionToComplete: empty or where stuck?
    pub problematic_elements: Vec<String>,
    pub problematic_state_hashes: Vec<u64>,
    // Path, i.e., Vec of transitions
    // counter_example: Vec<Transition>
}

impl GeneralPropertyResult {
    pub fn safe() -> GeneralPropertyResult {
        GeneralPropertyResult {
            property: GeneralProperty::Safeness,
            fulfilled: true,
            problematic_elements: vec![],
            problematic_state_hashes: vec![],
        }
    }
    pub fn always_terminates() -> GeneralPropertyResult {
        GeneralPropertyResult {
            property: GeneralProperty::OptionToComplete,
            fulfilled: true,
            problematic_elements: vec![],
            problematic_state_hashes: vec![],
        }
    }
    pub fn no_dead_activities() -> GeneralPropertyResult {
        GeneralPropertyResult {
            property: GeneralProperty::NoDeadActivities,
            fulfilled: true,
            problematic_elements: vec![],
            problematic_state_hashes: vec![],
        }
    }
}
