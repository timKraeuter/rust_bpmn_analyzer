use crate::bpmn::flow_node::FlowNodeType;
use crate::bpmn::process::Process;
use crate::bpmn::{
    add_terminated_state_hash_if_needed, check_properties, determine_properties, explore_state,
};
use crate::model_checking::bpmn_properties::{ModelCheckingResult, Property};
use crate::states::state_space::{ProcessSnapshot, State, StateSpace};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq)]
pub struct Collaboration {
    pub name: String,
    pub participants: Vec<Process>,
}

impl Collaboration {
    pub fn add_participant(&mut self, participant: Process) {
        self.participants.push(participant);
    }

    pub fn explore_state_space(
        &self,
        start_state: State,
        properties: Vec<Property>,
    ) -> ModelCheckingResult {
        let mut property_results = vec![];
        let mut not_executed_activities = self.get_all_flow_nodes_by_type(FlowNodeType::Task);

        let mut seen_state_hashes = HashMap::new();
        let start_state_hash = start_state.calc_hash();
        seen_state_hashes.insert(start_state_hash, true);

        let mut state_space = StateSpace {
            start_state_hash,
            terminated_state_hashes: vec![],
            states: HashMap::new(),
            transitions: HashMap::new(),
        };

        let mut unexplored_states = vec![(start_state_hash, start_state)];

        while !unexplored_states.is_empty() {
            match unexplored_states.pop() {
                None => {}
                Some((current_state_hash, current_state)) => {
                    // Explore the state
                    let potentially_unexplored_states =
                        explore_state(self, &current_state, &mut not_executed_activities);

                    // Check if we know the state already
                    let mut potentially_unexplored_states_hashes = vec![];
                    for (flow_node_id, new_state) in potentially_unexplored_states {
                        let new_hash = new_state.calc_hash();
                        match seen_state_hashes.get(&new_hash) {
                            None => {
                                // State is new.
                                seen_state_hashes.insert(new_hash, true);
                                unexplored_states.push((new_hash, new_state));
                            }
                            Some(_) => {}
                        }
                        potentially_unexplored_states_hashes.push((flow_node_id, new_hash));
                    }
                    // Do stuff for model checking
                    check_properties(
                        current_state_hash,
                        &current_state,
                        &properties,
                        &mut property_results,
                        &potentially_unexplored_states_hashes,
                    );
                    add_terminated_state_hash_if_needed(
                        current_state_hash,
                        &current_state,
                        &mut state_space,
                    );

                    // Save the state and its transitions.
                    state_space.states.insert(current_state_hash, current_state);
                    state_space
                        .transitions
                        .insert(current_state_hash, potentially_unexplored_states_hashes);
                }
            };
        }
        determine_properties(
            &properties,
            &mut property_results,
            &state_space,
            self,
            not_executed_activities,
        );

        ModelCheckingResult {
            state_space,
            property_results,
        }
    }

    pub(crate) fn get_all_flow_nodes_by_type(
        &self,
        flow_node_type: FlowNodeType,
    ) -> HashMap<String, bool> {
        let mut flow_nodes = HashMap::new();
        self.participants.iter().for_each(|process| {
            process
                .flow_nodes
                .iter()
                .filter(|flow_node| flow_node.flow_node_type == flow_node_type)
                .for_each(|flow_node| {
                    // Cloned id here. Could use RC smart pointer instead.
                    flow_nodes.insert(flow_node.id.clone(), true);
                })
        });
        flow_nodes
    }

    pub fn create_start_state(&self) -> State {
        let mut start = State { snapshots: vec![] };
        for process in &self.participants {
            let mut snapshot = ProcessSnapshot {
                // Cloning the string here could be done differently.
                id: process.id.clone(),
                tokens: BTreeMap::new(),
            };
            for flow_node in &process.flow_nodes {
                if flow_node.flow_node_type == FlowNodeType::StartEvent {
                    for out_sf in flow_node.outgoing_flows.iter() {
                        // Cloning the string here could be done differently.
                        let position = out_sf.id.clone();
                        snapshot.add_token(position);
                    }
                }
            }
            start.snapshots.push(snapshot);
        }
        start
    }
}
