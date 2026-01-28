//! State space exploration with partial order reduction.
//!
//! This module implements the state space exploration algorithm using
//! ample sets to reduce the number of states explored.

use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{EventType, FlowNodeType};
use crate::model_checking::por::ample_set::{AmpleSetConfig, AmpleSetStats, compute_ample_set};
use crate::model_checking::por::independence::TransitionEffect;
use crate::model_checking::por::result::ModelCheckingResultWithStats;
use crate::model_checking::properties::{
    ModelCheckingResult, Property, PropertyResult, check_on_the_fly_properties,
    determine_properties,
};
use crate::states::state_space::{State, StateSpace};
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet, VecDeque};

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
    explore_state_space_with_por(collaboration, properties, por_config)
}

/// Explore the state space using partial order reduction (ample sets).
///
/// This function reduces the state space by only exploring a representative
/// subset of interleavings, while preserving all properties.
///
/// # Arguments
/// * `collaboration` - The BPMN collaboration to analyze
/// * `properties` - The properties to check
/// * `por_config` - Configuration for partial order reduction
///
/// # Returns
/// A `ModelCheckingResultWithStats` containing the state space, property results,
/// and statistics about the reduction achieved.
pub fn explore_state_space_with_por<'a>(
    collaboration: &'a Collaboration,
    properties: Vec<Property>,
    por_config: AmpleSetConfig,
) -> ModelCheckingResultWithStats<'a> {
    let mut property_results = vec![];
    let mut not_executed_activities = collaboration.get_all_tasks();
    let mut ample_stats = AmpleSetStats::default();

    let mut seen_state_hashes: HashMap<u64, bool> = HashMap::new();
    let start_state = collaboration.create_start_state();
    let start_state_hash = start_state.calc_hash();
    seen_state_hashes.insert(start_state_hash, true);

    let mut state_space = StateSpace {
        start_state_hash,
        terminated_state_hashes: vec![],
        states: HashMap::new(),
        transitions: HashMap::new(),
    };

    let mut unexplored_states = VecDeque::new();
    unexplored_states.push_back((start_state_hash, start_state));

    // Track states that need re-exploration with full expansion (for sticky proviso)
    let mut needs_full_expansion: HashSet<u64> = HashSet::new();

    while let Some((current_state_hash, current_state)) = unexplored_states.pop_front() {
        // Get all enabled transitions with their effects
        let enabled_transitions = get_enabled_transitions(collaboration, &current_state);

        // Check if this state needs full expansion due to sticky proviso
        let force_full_expansion = needs_full_expansion.contains(&current_state_hash);

        // Compute the ample set
        let ample_result = compute_ample_set(
            &enabled_transitions,
            &por_config,
            false, // BFS doesn't have a stack
            force_full_expansion,
        );

        // Record statistics
        ample_stats.record(&ample_result, enabled_transitions.len());

        // Execute only the transitions in the ample set
        let selected_flow_node_ids: HashSet<&str> = ample_result
            .transitions
            .iter()
            .map(|t| t.flow_node_id)
            .collect();

        let potentially_unexplored_states = explore_state_filtered(
            collaboration,
            &current_state,
            &mut not_executed_activities,
            &selected_flow_node_ids,
        );

        let mut transitions = vec![];
        let mut has_back_edge = false;

        for (flow_node_id, new_state) in potentially_unexplored_states {
            let new_hash = new_state.calc_hash();
            // Check if we know the state already
            if let Vacant(e) = seen_state_hashes.entry(new_hash) {
                // State is new.
                e.insert(true);
                unexplored_states.push_back((new_hash, new_state));
            } else {
                // This is a back edge to an already-seen state
                has_back_edge = true;
            }
            // Remember states to make transitions.
            transitions.push((flow_node_id, new_hash));
        }

        // Sticky proviso: If we have a back edge and didn't fully expand,
        // we need to re-explore this state with full expansion to ensure correctness.
        // However, re-adding to queue would cause issues, so instead we ensure
        // that the target of back edges gets fully expanded.
        if has_back_edge && !ample_result.is_full_expansion {
            // Mark this state for full expansion if we see it again
            // (This is a simplified version - a more complete implementation
            // would re-explore immediately)
            needs_full_expansion.insert(current_state_hash);

            // Also, we should explore all transitions now to be safe
            // Re-explore with all enabled transitions
            let all_flow_node_ids: HashSet<&str> =
                enabled_transitions.iter().map(|t| t.flow_node_id).collect();

            // Get the missing transitions (those not in ample set)
            let missing_flow_node_ids: HashSet<&str> = all_flow_node_ids
                .difference(&selected_flow_node_ids)
                .copied()
                .collect();

            if !missing_flow_node_ids.is_empty() {
                let additional_states = explore_state_filtered(
                    collaboration,
                    &current_state,
                    &mut not_executed_activities,
                    &missing_flow_node_ids,
                );

                for (flow_node_id, new_state) in additional_states {
                    let new_hash = new_state.calc_hash();
                    if let Vacant(e) = seen_state_hashes.entry(new_hash) {
                        e.insert(true);
                        unexplored_states.push_back((new_hash, new_state));
                    }
                    transitions.push((flow_node_id, new_hash));
                }
            }
        }

        // Do stuff for model checking
        let property_result = check_on_the_fly_properties(
            current_state_hash,
            &current_state,
            &properties,
            &mut property_results,
            &transitions,
        );
        state_space.mark_terminated_if_needed(&current_state, current_state_hash);

        // Save the state and its transitions.
        state_space.states.insert(current_state_hash, current_state);
        if !transitions.is_empty() {
            state_space
                .transitions
                .insert(current_state_hash, transitions);
        }

        match property_result {
            Ok(_) => {}
            Err(e) => {
                property_results.push(PropertyResult {
                    property: Property::OptionToComplete,
                    problematic_state_hashes: vec![current_state_hash],
                    problematic_elements: vec![e.overflowing_position],
                    fulfilled: false,
                });
                break;
            }
        }
    }

    determine_properties(
        &properties,
        &mut property_results,
        not_executed_activities,
        &state_space,
    );

    ModelCheckingResultWithStats {
        result: ModelCheckingResult {
            state_space,
            property_results,
        },
        ample_set_stats: ample_stats,
    }
}

/// Get all enabled transitions with their effects for the current state.
fn get_enabled_transitions<'a>(
    collaboration: &'a Collaboration,
    state: &State<'a>,
) -> Vec<TransitionEffect<'a>> {
    let mut effects = vec![];

    // Check message start events
    if !state.messages.is_empty() {
        for process in &collaboration.participants {
            for flow_node in &process.flow_nodes {
                if flow_node.flow_node_type == FlowNodeType::StartEvent(EventType::Message) {
                    // Create an effect for message start events
                    let mut effect = TransitionEffect::new(&flow_node.id, &process.id);
                    for inc_mf in &flow_node.incoming_message_flows {
                        if state.messages.contains_key(inc_mf.id.as_str()) {
                            effect.consumes_messages.insert(inc_mf.id.as_str());
                        }
                    }
                    if !effect.consumes_messages.is_empty() {
                        for out_flow in &flow_node.outgoing_flows {
                            effect.produces_tokens.insert(out_flow.id.as_str());
                        }
                        effects.push(effect);
                    }
                }
            }
        }
    }

    // Check regular flow nodes
    for snapshot in &state.snapshots {
        let process = collaboration
            .participants
            .iter()
            .find(|process| process.id == snapshot.id);

        if let Some(process) = process {
            for flow_node in
                Collaboration::get_flow_node_indexes_with_incoming_tokens(snapshot, process)
                    .iter()
                    .filter_map(|&flow_node_idx| process.flow_nodes.get(*flow_node_idx))
            {
                if let Some(effect) =
                    flow_node.get_transition_effect(&process.id, snapshot, state, process)
                {
                    effects.push(effect);
                }
            }
        }
    }

    effects
}

/// Explore state but only execute transitions for flow nodes in the filter set.
fn explore_state_filtered<'a>(
    collaboration: &'a Collaboration,
    state: &State<'a>,
    not_executed_activities: &mut HashMap<&str, bool>,
    selected_flow_nodes: &HashSet<&str>,
) -> Vec<(&'a str, State<'a>)> {
    let mut unexplored_states: Vec<(&str, State)> = vec![];

    // Handle message start events
    if !state.messages.is_empty() {
        try_trigger_message_start_events_filtered(
            collaboration,
            state,
            &mut unexplored_states,
            selected_flow_nodes,
        );
    }

    for snapshot in &state.snapshots {
        let process = collaboration
            .participants
            .iter()
            .find(|process| process.id == snapshot.id);

        match process {
            None => {
                panic!("No process found for snapshot with id \"{}\"", snapshot.id)
            }
            Some(process) => {
                for flow_node in
                    Collaboration::get_flow_node_indexes_with_incoming_tokens(snapshot, process)
                        .iter()
                        .filter_map(|&flow_node_idx| process.flow_nodes.get(*flow_node_idx))
                        .filter(|flow_node| selected_flow_nodes.contains(flow_node.id.as_str()))
                {
                    let new_states =
                        flow_node.try_execute(snapshot, state, process, not_executed_activities);

                    Collaboration::record_executed_activities(
                        not_executed_activities,
                        flow_node,
                        &new_states,
                    );

                    unexplored_states.append(
                        &mut new_states
                            .into_iter()
                            .map(|state| (flow_node.id.as_str(), state))
                            .collect(),
                    );
                }
            }
        }
    }
    unexplored_states
}

/// Try to trigger message start events, but only for selected flow nodes.
fn try_trigger_message_start_events_filtered<'a>(
    collaboration: &'a Collaboration,
    state: &State<'a>,
    unexplored_states: &mut Vec<(&'a str, State<'a>)>,
    selected_flow_nodes: &HashSet<&str>,
) {
    collaboration.participants.iter().for_each(|process| {
        process
            .flow_nodes
            .iter()
            .filter(|flow_node| {
                flow_node.flow_node_type == FlowNodeType::StartEvent(EventType::Message)
            })
            .filter(|flow_node| selected_flow_nodes.contains(flow_node.id.as_str()))
            .for_each(|message_start_event| {
                let new_states =
                    message_start_event.try_trigger_message_start_event(process, state);
                unexplored_states.append(
                    &mut new_states
                        .into_iter()
                        .map(|state| (message_start_event.id.as_str(), state))
                        .collect(),
                );
            })
    });
}
