use crate::states::state_space::{State, StateSpace};
use clap::ValueEnum;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Property {
    Safeness,
    OptionToComplete,
    ProperCompletion,
    NoDeadActivities,
}

#[derive(Debug)]
pub struct ModelCheckingResult {
    pub state_space: StateSpace,
    pub property_results: Vec<PropertyResult>,
}

impl ModelCheckingResult {
    pub fn get_state(&self, state_hash: &u64) -> Option<&State> {
        self.state_space.states.get(state_hash)
    }
}

#[derive(Debug, PartialEq)]
pub struct PropertyResult {
    pub property: Property,
    pub fulfilled: bool,
    // DeadActivities: Dead activities
    // Safeness: Unsafe sfs, ProperCompletion: End Events executed twice, OptionToComplete: empty
    pub problematic_elements: Vec<String>,
    // Safeness: Unsafe states, ProperCompletion: Unproper states, OptionToComplete: Stuck states.
    pub problematic_state_hashes: Vec<u64>,
}

impl Default for PropertyResult {
    fn default() -> Self {
        PropertyResult {
            property: Property::Safeness,
            fulfilled: false,
            problematic_elements: vec![],
            problematic_state_hashes: vec![],
        }
    }
}

impl PropertyResult {
    pub fn safe() -> PropertyResult {
        PropertyResult {
            property: Property::Safeness,
            fulfilled: true,
            ..Default::default()
        }
    }
    pub fn always_terminates() -> PropertyResult {
        PropertyResult {
            property: Property::OptionToComplete,
            fulfilled: true,
            ..Default::default()
        }
    }
    pub fn no_dead_activities() -> PropertyResult {
        PropertyResult {
            property: Property::NoDeadActivities,
            fulfilled: true,
            ..Default::default()
        }
    }
    pub fn proper_completion() -> PropertyResult {
        PropertyResult {
            property: Property::ProperCompletion,
            fulfilled: true,
            ..Default::default()
        }
    }
}

pub fn determine_properties(
    properties: &[Property],
    results: &mut Vec<PropertyResult>,
    never_executed_activities: HashMap<String, bool>,
    state_space: &StateSpace,
) {
    if properties.contains(&Property::Safeness)
        && not_contains_property_result(results, Property::Safeness)
    {
        results.push(PropertyResult::safe());
    }
    if properties.contains(&Property::OptionToComplete)
        && not_contains_property_result(results, Property::OptionToComplete)
    {
        results.push(PropertyResult::always_terminates())
    }
    if properties.contains(&Property::ProperCompletion) {
        state_space
            .terminated_state_hashes
            .iter()
            .for_each(|terminated_state_hash| {
                let state = state_space.states.get(terminated_state_hash).unwrap();
                for (end_event, count) in state.executed_end_event_counter.iter() {
                    if count > &1u16 {
                        record_proper_completion(*terminated_state_hash, results, end_event);
                    }
                }
            });
        if not_contains_property_result(results, Property::ProperCompletion) {
            results.push(PropertyResult::proper_completion())
        }
    }
    if properties.contains(&Property::NoDeadActivities) {
        // Cannot do this in the loop due to the borrow checker.
        let mut dead_activities: Vec<String> = never_executed_activities.into_keys().collect();
        if !dead_activities.is_empty() {
            dead_activities.sort();
            results.push(PropertyResult {
                property: Property::NoDeadActivities,
                fulfilled: false,
                problematic_elements: dead_activities,
                ..Default::default()
            });
        } else {
            results.push(PropertyResult::no_dead_activities());
        }
    }
}

fn not_contains_property_result(results: &[PropertyResult], property: Property) -> bool {
    !results.iter().any(|result| result.property == property)
}

pub fn check_on_the_fly_properties(
    current_state_hash: u64,
    state: &State,
    properties: &[Property],
    results: &mut Vec<PropertyResult>,
    next_state_hashes: &Vec<(String, u64)>,
) {
    for property in properties.iter() {
        match property {
            Property::Safeness => {
                check_if_unsafe(current_state_hash, state, results);
            }
            Property::OptionToComplete => {
                check_if_stuck(current_state_hash, state, results, next_state_hashes)
            }
            _ => {}
        }
    }
}

pub fn check_if_stuck(
    current_state_hash: u64,
    state: &State,
    results: &mut Vec<PropertyResult>,
    next_state_hashes: &Vec<(String, u64)>,
) {
    if next_state_hashes.is_empty() && !state.is_terminated() {
        record_option_to_complete(current_state_hash, results);
    }
}

pub fn record_option_to_complete(current_state_hash: u64, results: &mut Vec<PropertyResult>) {
    match find_property_result(results, Property::OptionToComplete) {
        None => results.push(PropertyResult {
            property: Property::OptionToComplete,
            fulfilled: false,
            problematic_state_hashes: vec![current_state_hash],
            ..Default::default()
        }),
        Some(result) => result.problematic_state_hashes.push(current_state_hash),
    }
}

fn record_proper_completion(
    current_state_hash: u64,
    results: &mut Vec<PropertyResult>,
    end_event: &str,
) {
    match find_property_result(results, Property::ProperCompletion) {
        None => results.push(PropertyResult {
            property: Property::ProperCompletion,
            fulfilled: false,
            problematic_elements: vec![end_event.to_owned()],
            problematic_state_hashes: vec![current_state_hash],
            ..Default::default()
        }),
        Some(result) => {
            result.problematic_elements.push(end_event.to_owned());
            result.problematic_state_hashes.push(current_state_hash);
        }
    }
}

fn find_property_result(
    results: &mut [PropertyResult],
    property: Property,
) -> Option<&mut PropertyResult> {
    results
        .iter_mut()
        .find(|result| result.property == property)
}

fn check_if_unsafe(current_state_hash: u64, state: &State, results: &mut Vec<PropertyResult>) {
    match state.get_unsafe_sf() {
        None => {}
        Some(unsafe_sf) => match find_property_result(results, Property::Safeness) {
            None => results.push(PropertyResult {
                property: Property::Safeness,
                fulfilled: false,
                problematic_elements: vec![unsafe_sf.clone()],
                problematic_state_hashes: vec![current_state_hash],
                ..Default::default()
            }),
            Some(result) => {
                result.problematic_elements.push(unsafe_sf.clone());
                result.problematic_state_hashes.push(current_state_hash)
            }
        },
    }
}
