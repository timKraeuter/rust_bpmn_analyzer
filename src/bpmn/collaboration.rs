use crate::bpmn::flow_node::FlowNodeType::StartEvent;
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType, MessageFlow, TaskType};
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
    pub fn add_message_flow(&mut self, mf_id: String, mf_source: String, mf_target: String) {
        // Could be optimized by stopping if source and target were added.
        self.participants.iter_mut().for_each(|process| {
            for flow_node in process.flow_nodes.iter_mut() {
                if flow_node.id == mf_source {
                    flow_node.add_outgoing_message_flow(MessageFlow { id: mf_id.clone() });
                    continue;
                }
                if flow_node.id == mf_target {
                    flow_node.add_incoming_message_flow(MessageFlow { id: mf_id.clone() });
                }
            }
        });
    }
    pub fn add_participant(&mut self, participant: Process) {
        self.participants.push(participant);
    }

    pub fn explore_state_space(&self, properties: Vec<Property>) -> ModelCheckingResult {
        let mut property_results = vec![];
        let mut not_executed_activities = self.get_all_tasks();

        let mut seen_state_hashes = HashMap::new();
        let start_state = self.create_start_state();
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

                    let mut transitions = vec![];
                    for (flow_node_id, new_state) in potentially_unexplored_states {
                        let new_hash = new_state.calc_hash();
                        // Check if we know the state already
                        match seen_state_hashes.get(&new_hash) {
                            None => {
                                // State is new.
                                seen_state_hashes.insert(new_hash, true);
                                unexplored_states.push((new_hash, new_state));
                            }
                            Some(_) => {}
                        }
                        // Remember states to make transitions.
                        transitions.push((flow_node_id, new_hash));
                    }
                    // Do stuff for model checking
                    check_on_the_fly_properties(
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

    pub fn get_all_tasks(&self) -> HashMap<String, bool> {
        let mut flow_nodes = HashMap::new();
        self.participants.iter().for_each(|process| {
            process
                .flow_nodes
                .iter()
                .filter(|flow_node| {
                    flow_node.flow_node_type == FlowNodeType::Task(TaskType::Default)
                        || flow_node.flow_node_type == FlowNodeType::Task(TaskType::Receive)
                })
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
            messages: BTreeMap::new(),
        };
        for process in &self.participants {
            let mut tokens = BTreeMap::new();
            for flow_node in &process.flow_nodes {
                if flow_node.flow_node_type == FlowNodeType::StartEvent(EventType::None) {
                    for out_sf in flow_node.outgoing_flows.iter() {
                        // Cloning the string here could be done differently.
                        tokens.insert(out_sf.id.as_str(), 1);
                    }
                }
            }
            if !tokens.is_empty() {
                start.snapshots.push(ProcessSnapshot {
                    id: &process.id,
                    tokens,
                });
            }
        }
        start
    }

    fn explore_state<'a>(
        &'a self,
        state: &State<'a>,
        not_executed_activities: &mut HashMap<String, bool>,
    ) -> Vec<(String, State<'a>)> {
        let mut unexplored_states: Vec<(String, State)> = vec![];
        if !state.messages.is_empty() {
            self.try_trigger_message_start_events(state, &mut unexplored_states);
        }

        for snapshot in &state.snapshots {
            // Find participant for snapshot, could also be hashmap but usually not a long list.
            let process = self
                .participants
                .iter()
                .find(|process_snapshot| process_snapshot.id == snapshot.id);
            match process {
                None => {
                    panic!("No process found for snapshot with id \"{}\"", snapshot.id)
                }
                Some(process) => {
                    // TODO: Would be nice to only try to execute flow nodes that have incoming tokens/messages. But currently sfs/mfs are just ids and we cannot find their targets easily.
                    for flow_node in process.flow_nodes.iter() {
                        let new_states = flow_node.try_execute(snapshot, state, self);

                        Self::record_executed_activities(
                            not_executed_activities,
                            flow_node,
                            &new_states,
                        );

                        // Would want to check if the state has been explored here not later to not take up unnecessary memory. But we still want to add the transitions.
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

    fn try_trigger_message_start_events<'a>(
        &'a self,
        state: &State<'a>,
        unexplored_states: &mut Vec<(String, State<'a>)>,
    ) {
        self.participants.iter().for_each(|process| {
            process
                .flow_nodes
                .iter()
                .filter(|flow_node| flow_node.flow_node_type == StartEvent(EventType::Message))
                .for_each(|message_start_event| {
                    let new_states =
                        message_start_event.try_trigger_message_start_event(process, state);
                    // Would want to check if the state has been explored here not later to not take up unnecessary memory. But we still want to add the transitions.
                    unexplored_states.append(
                        &mut new_states
                            .into_iter()
                            .map(|state| (message_start_event.id.clone(), state))
                            .collect(),
                    );
                })
        });
    }

    fn record_executed_activities(
        not_executed_activities: &mut HashMap<String, bool>,
        flow_node: &FlowNode,
        new_states: &[State],
    ) {
        if (flow_node.flow_node_type == FlowNodeType::Task(TaskType::Default)
            || flow_node.flow_node_type == FlowNodeType::Task(TaskType::Receive))
            && not_executed_activities.get(&flow_node.id).is_some()
            && !new_states.is_empty()
        {
            not_executed_activities.remove(&flow_node.id);
        }
    }
}
