use crate::bpmn::flow_node::{FlowNode, FlowNodeType};
use crate::bpmn::process::Process;
use crate::model_checking::properties::{
    check_on_the_fly_properties, determine_properties, ModelCheckingResult, Property,
};
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
                        self.explore_state(&current_state, &mut not_executed_activities);

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
                    check_on_the_fly_properties(
                        current_state_hash,
                        &current_state,
                        &properties,
                        &mut property_results,
                        &potentially_unexplored_states_hashes,
                    );
                    state_space.mark_terminated_if_needed(&current_state, current_state_hash);

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
            not_executed_activities,
            &state_space,
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
        let mut start = State {
            snapshots: vec![],
            executed_end_event_counter: BTreeMap::new(),
        };
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

    fn explore_state(
        &self,
        state: &State,
        not_executed_activities: &mut HashMap<String, bool>,
    ) -> Vec<(String, State)> {
        let mut unexplored_states: Vec<(String, State)> = vec![];
        for snapshot in &state.snapshots {
            // Find participant for snapshot, could also be hashmap but usually not a long list.
            let option = self
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

                        Self::record_executed_activities(
                            not_executed_activities,
                            flow_node,
                            &new_states,
                        );

                        // Would want to check if the state has been explored here not later to not take up unnecessary memory.
                        unexplored_states.append(
                            &mut new_states
                                .into_iter()
                                .map(|state| (flow_node.id.clone(), state))
                                .collect(),
                        );
                    }
                }
            }
        }
        unexplored_states
    }

    fn record_executed_activities(
        not_executed_activities: &mut HashMap<String, bool>,
        flow_node: &FlowNode,
        new_states: &Vec<State>,
    ) {
        if flow_node.flow_node_type == FlowNodeType::Task
            && !new_states.is_empty()
            && !not_executed_activities.is_empty()
        {
            not_executed_activities.remove(&flow_node.id);
        }
    }
}