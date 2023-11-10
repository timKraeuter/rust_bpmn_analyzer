use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{FlowNode, FlowNodeType};
use crate::bpmn::process::Process;
use crate::model_checking::bpmn_properties::{Property, PropertyResult};
use crate::states::state_space::{State, StateSpace};
pub use reader::read_bpmn_file;
use std::collections::HashMap;

mod collaboration;
mod flow_node;
mod process;
mod reader;
mod test;

fn determine_properties(
    properties: &[Property],
    results: &mut Vec<PropertyResult>,
    state_space: &StateSpace,
    collaboration: &Collaboration,
    never_executed_activities: HashMap<String, bool>,
) {
    if properties.contains(&Property::Safeness)
        && contains_property_result(results, Property::Safeness)
    {
        results.push(PropertyResult::safe());
    }
    if properties.contains(&Property::OptionToComplete)
        && contains_property_result(results, Property::OptionToComplete)
    {
        results.push(PropertyResult::always_terminates())
    }
    if properties.contains(&Property::ProperCompletion) {
        // TODO: This can loop and never end
        check_proper_completion(collaboration, state_space, results);
    }
    if properties.contains(&Property::NoDeadActivities) {
        // Cannot do this in the loop due to the borrow checker.
        // TODO: This can loop and never end
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

fn check_proper_completion(
    collaboration: &Collaboration,
    state_space: &StateSpace,
    results: &mut Vec<PropertyResult>,
) {
    let end_events: Vec<String> = collaboration
        .get_all_flow_nodes_by_type(FlowNodeType::EndEvent)
        .into_keys()
        .collect();

    match state_space.transitions.get(&state_space.start_state_hash) {
        None => {}
        Some(transitions) => {
            for (flow_node, next_state_hash) in transitions {
                match check_if_end_event_executed_twice(
                    flow_node,
                    next_state_hash,
                    state_space,
                    HashMap::new(),
                    &end_events,
                ) {
                    None => {}
                    Some((end_event, path)) => {
                        results.push(PropertyResult {
                            property: Property::ProperCompletion,
                            fulfilled: false,
                            problematic_elements: vec![end_event],
                            counter_example: path,
                            ..Default::default()
                        });
                        return;
                    }
                };
            }
        }
    }

    results.push(PropertyResult::proper_completion());
}

fn check_if_end_event_executed_twice(
    flow_node: &String,
    current_state: &u64,
    state_space: &StateSpace,
    mut seen_end_events: HashMap<String, bool>,
    all_end_events: &Vec<String>,
) -> Option<(String, Vec<(String, u64)>)> {
    if all_end_events.contains(flow_node) {
        match seen_end_events.get(flow_node) {
            None => {
                seen_end_events.insert(flow_node.clone(), true);
            }
            Some(_) => {
                return Some((flow_node.clone(), vec![(flow_node.clone(), *current_state)]));
            }
        }
    }
    match state_space.transitions.get(current_state) {
        None => {}
        Some(transitions) => {
            for (next_flow_node, next_state_hash) in transitions {
                match check_if_end_event_executed_twice(
                    next_flow_node,
                    next_state_hash,
                    state_space,
                    seen_end_events.clone(),
                    all_end_events,
                ) {
                    None => {}
                    Some((end_event, mut path)) => {
                        path.insert(0, (flow_node.clone(), *current_state));
                        return Some((end_event, path));
                    }
                };
            }
        }
    }
    None
}

fn contains_property_result(results: &[PropertyResult], property: Property) -> bool {
    !results.iter().any(|result| result.property == property)
}

fn check_properties(
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

fn check_if_stuck(
    current_state_hash: u64,
    state: &State,
    results: &mut Vec<PropertyResult>,
    next_state_hashes: &Vec<(String, u64)>,
) {
    if next_state_hashes.is_empty() && !state.is_terminated() {
        match results
            .iter_mut()
            .find(|result| result.property == Property::OptionToComplete)
        {
            None => results.push(PropertyResult {
                property: Property::OptionToComplete,
                fulfilled: false,
                problematic_state_hashes: vec![current_state_hash],
                ..Default::default()
            }),
            Some(result) => result.problematic_state_hashes.push(current_state_hash),
        }
    }
}

fn check_if_unsafe(current_state_hash: u64, state: &State, results: &mut Vec<PropertyResult>) {
    const TWO: u16 = 2;
    for snapshot in &state.snapshots {
        match snapshot.tokens.iter().find(|(_, amount)| *amount >= &TWO) {
            None => {}
            Some((unsafe_flow_element, _)) => results.push(PropertyResult {
                property: Property::Safeness,
                fulfilled: false,
                problematic_elements: vec![unsafe_flow_element.clone()],
                problematic_state_hashes: vec![current_state_hash],
                ..Default::default()
            }),
        }
    }
}

fn explore_state(
    collaboration: &Collaboration,
    state: &State,
    not_executed_activities: &mut HashMap<String, bool>,
) -> Vec<(String, State)> {
    let mut unexplored_states: Vec<(String, State)> = vec![];
    for snapshot in &state.snapshots {
        // Find participant for snapshot, could also be hashmap but usually not a long list.
        let option = collaboration
            .participants
            .iter()
            .find(|process_snapshot| process_snapshot.id == snapshot.id);
        match option {
            None => {
                panic!("No process found for snapshot with id \"{}\"", snapshot.id)
            }
            Some(matching_process) => {
                for flow_node in matching_process.flow_nodes.iter() {
                    let new_states = flow_node.try_execute(snapshot, state);

                    // Record activity execution
                    if flow_node.flow_node_type == FlowNodeType::Task
                        && !new_states.is_empty()
                        && !not_executed_activities.is_empty()
                    {
                        not_executed_activities.remove(&flow_node.id);
                    }

                    // Would want to check if the state has been explored here not later to not take up unnecessary memory.
                    unexplored_states.append(
                        &mut new_states
                            .into_iter()
                            // Add info about flow node. Should use Rc instead of clone in the future.
                            .map(|state| (flow_node.id.clone(), state))
                            .collect(),
                    );
                }
            }
        }
    }
    unexplored_states
}
