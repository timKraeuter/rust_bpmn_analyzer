use crate::states::state_space::{State, StateSpace};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
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
    property_results: &mut Vec<PropertyResult>,
    never_executed_activities: HashMap<String, bool>,
    state_space: &StateSpace,
) {
    if properties.contains(&Property::Safeness)
        && not_contains_property_result(property_results, Property::Safeness)
    {
        property_results.push(PropertyResult::safe());
    }
    if properties.contains(&Property::OptionToComplete)
        && not_contains_property_result(property_results, Property::OptionToComplete)
    {
        property_results.push(PropertyResult::always_terminates())
    }
    if properties.contains(&Property::ProperCompletion) {
        state_space
            .terminated_state_hashes
            .iter()
            .for_each(|terminated_state_hash| {
                let state = state_space.states.get(terminated_state_hash).unwrap();
                for (end_event, count) in state.executed_end_event_counter.iter() {
                    if count > &1u16 {
                        record_proper_completion(
                            *terminated_state_hash,
                            property_results,
                            end_event,
                        );
                    }
                }
            });
        if not_contains_property_result(property_results, Property::ProperCompletion) {
            property_results.push(PropertyResult::proper_completion())
        }
    }
    if properties.contains(&Property::NoDeadActivities) {
        // Cannot do this in the loop due to the borrow checker.
        let mut dead_activities: Vec<String> = never_executed_activities.into_keys().collect();
        if !dead_activities.is_empty() {
            dead_activities.sort();
            property_results.push(PropertyResult {
                property: Property::NoDeadActivities,
                fulfilled: false,
                problematic_elements: dead_activities,
                ..Default::default()
            });
        } else {
            property_results.push(PropertyResult::no_dead_activities());
        }
    }
}

fn not_contains_property_result(property_results: &[PropertyResult], property: Property) -> bool {
    !property_results
        .iter()
        .any(|result| result.property == property)
}

pub fn check_on_the_fly_properties(
    current_state_hash: u64,
    current_state: &State,
    properties: &[Property],
    property_results: &mut Vec<PropertyResult>,
    transitions: &[(String, u64)],
) {
    for property in properties.iter() {
        match property {
            Property::Safeness => {
                check_if_unsafe(current_state_hash, current_state, property_results);
            }
            Property::OptionToComplete => check_if_stuck(
                current_state_hash,
                current_state,
                property_results,
                transitions,
            ),
            _ => {}
        }
    }
}

pub fn check_if_stuck(
    current_state_hash: u64,
    current_state: &State,
    property_results: &mut Vec<PropertyResult>,
    transitions: &[(String, u64)],
) {
    if transitions.is_empty() && !current_state.is_terminated() {
        record_option_to_complete(current_state_hash, property_results);
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
    property_results: &mut Vec<PropertyResult>,
    end_event: &str,
) {
    match find_property_result(property_results, Property::ProperCompletion) {
        None => property_results.push(PropertyResult {
            property: Property::ProperCompletion,
            fulfilled: false,
            problematic_elements: vec![end_event.to_owned()],
            problematic_state_hashes: vec![current_state_hash],
        }),
        Some(result) => {
            result.problematic_elements.push(end_event.to_owned());
            result.problematic_state_hashes.push(current_state_hash);
        }
    }
}

fn find_property_result(
    property_results: &mut [PropertyResult],
    property: Property,
) -> Option<&mut PropertyResult> {
    property_results
        .iter_mut()
        .find(|result| result.property == property)
}

fn check_if_unsafe(
    current_state_hash: u64,
    current_state: &State,
    property_results: &mut Vec<PropertyResult>,
) {
    match current_state.try_find_unsafe_sf_id() {
        None => {}
        Some(unsafe_sf) => match find_property_result(property_results, Property::Safeness) {
            None => property_results.push(PropertyResult {
                property: Property::Safeness,
                fulfilled: false,
                problematic_elements: vec![unsafe_sf.clone()],
                problematic_state_hashes: vec![current_state_hash],
            }),
            Some(result) => {
                result.problematic_elements.push(unsafe_sf.clone());
                result.problematic_state_hashes.push(current_state_hash)
            }
        },
    }
}
