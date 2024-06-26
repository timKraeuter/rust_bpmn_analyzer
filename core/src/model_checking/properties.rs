use crate::states::state_space::{State, StateSpace};
use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Property {
    Safeness,
    OptionToComplete,
    ProperCompletion,
    NoDeadActivities,
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_property_string(self))
    }
}

fn get_property_string(property: &Property) -> ColoredString {
    match property {
        Property::Safeness => "Safeness".blue().bold(),
        Property::OptionToComplete => "Option to complete".blue().bold(),
        Property::ProperCompletion => "Proper completion".blue().bold(),
        Property::NoDeadActivities => "No dead activities".blue().bold(),
    }
}

#[derive(Debug)]
pub struct ModelCheckingResult<'a> {
    pub state_space: StateSpace<'a>,
    pub property_results: Vec<PropertyResult>,
}

impl ModelCheckingResult<'_> {
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
    never_executed_activities: HashMap<&str, bool>,
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
        if !never_executed_activities.is_empty() {
            let mut dead_activities: Vec<String> = never_executed_activities
                .keys()
                .map(|activity_id| activity_id.to_string())
                .collect();
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
    transitions: &[(&str, u64)],
) -> Result<(), LiveLockError> {
    for property in properties.iter() {
        match property {
            Property::Safeness => {
                check_if_unsafe_or_livelock(current_state_hash, current_state, property_results)?;
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
    Ok(())
}

pub fn check_if_stuck(
    current_state_hash: u64,
    current_state: &State,
    property_results: &mut Vec<PropertyResult>,
    transitions: &[(&str, u64)],
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

fn check_if_unsafe_or_livelock(
    current_state_hash: u64,
    current_state: &State,
    property_results: &mut Vec<PropertyResult>,
) -> Result<(), LiveLockError> {
    let unsafe_sfs = current_state.find_unsafe_sf_ids_or_livelock()?;
    if !unsafe_sfs.is_empty() {
        let unsafe_sfs = unsafe_sfs.iter().map(|&id| String::from(id)).collect();
        match find_property_result(property_results, Property::Safeness) {
            None => property_results.push(PropertyResult {
                property: Property::Safeness,
                fulfilled: false,
                problematic_elements: unsafe_sfs,
                problematic_state_hashes: vec![current_state_hash],
            }),
            Some(result) => {
                // TODO: We can end up with the same unsafe element multiple times coming from different state hashes. Problematic state hashes should be a tuple which has a list of problematic elements.
                result.problematic_elements.extend(unsafe_sfs);
                result.problematic_state_hashes.push(current_state_hash)
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct LiveLockError {
    pub overflowing_position: String,
}

impl Display for LiveLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Livelock (> MAX_TOKENS) found at BPMN element: {:?}",
            self.overflowing_position
        )
    }
}

impl std::error::Error for LiveLockError {}
