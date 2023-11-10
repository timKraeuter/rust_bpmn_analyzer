use crate::states::state_space::{State, StateSpace};
use clap::ValueEnum;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum BPMNProperty {
    Safeness,
    OptionToComplete,
    ProperCompletion,
    NoDeadActivities,
}

#[derive(Debug)]
pub struct ModelCheckingResult {
    pub state_space: StateSpace,
    pub property_results: Vec<BPMNPropertyResult>,
}

impl ModelCheckingResult {
    pub fn get_state(&self, state_hash: &u64) -> Option<&State> {
        self.state_space.states.get(state_hash)
    }
    pub fn get_path_to_state(&self, state_hash: u64) -> Option<Vec<(String, u64)>> {
        self.get_path(self.state_space.start_state_hash, state_hash)
    }
    fn get_path(&self, from_state_hash: u64, to_state_hash: u64) -> Option<Vec<(String, u64)>> {
        match self.state_space.transitions.get(&from_state_hash) {
            None => None,
            Some(next_states) => {
                match next_states
                    .iter()
                    .find(|(_, next_state_hash)| next_state_hash == &to_state_hash)
                {
                    None => {}
                    // Should be Rc in the future.
                    Some(last_transition) => return Some(vec![last_transition.clone()]),
                }
                // Not found continue searching
                for (flow_node_id, next_state) in next_states {
                    match self.get_path(*next_state, to_state_hash) {
                        None => {}
                        Some(mut path) => {
                            // Should be Rc in the future.
                            path.insert(0, (flow_node_id.clone(), from_state_hash));
                            return Some(path);
                        }
                    };
                }
                None
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BPMNPropertyResult {
    pub property: BPMNProperty,
    pub fulfilled: bool,
    // DeadActivities: Dead activities
    // Safeness: Unsafe sequence flows
    // OptionToComplete: empty or where stuck?
    pub problematic_elements: Vec<String>,
    pub problematic_state_hashes: Vec<u64>,
    // Path for option_to_complete
    pub counter_example: Vec<(String, u64)>,
}

impl Default for BPMNPropertyResult {
    fn default() -> Self {
        BPMNPropertyResult {
            property: BPMNProperty::Safeness,
            fulfilled: false,
            problematic_elements: vec![],
            problematic_state_hashes: vec![],
            counter_example: vec![],
        }
    }
}

impl BPMNPropertyResult {
    pub fn safe() -> BPMNPropertyResult {
        BPMNPropertyResult {
            property: BPMNProperty::Safeness,
            fulfilled: true,
            ..Default::default()
        }
    }
    pub fn always_terminates() -> BPMNPropertyResult {
        BPMNPropertyResult {
            property: BPMNProperty::OptionToComplete,
            fulfilled: true,
            ..Default::default()
        }
    }
    pub fn no_dead_activities() -> BPMNPropertyResult {
        BPMNPropertyResult {
            property: BPMNProperty::NoDeadActivities,
            fulfilled: true,
            ..Default::default()
        }
    }
    pub fn proper_completion() -> BPMNPropertyResult {
        BPMNPropertyResult {
            property: BPMNProperty::ProperCompletion,
            fulfilled: true,
            ..Default::default()
        }
    }
}
