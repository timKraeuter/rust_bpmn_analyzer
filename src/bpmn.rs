use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
pub use reader::read_bpmn_file;
pub use state_space::StateSpace;
pub use model_checking::{GeneralProperty, ModelCheckingResult, GeneralPropertyResult};

use crate::bpmn::state_space::{ProcessSnapshot, State};

mod reader;
mod state_space;
mod test;
mod model_checking;

#[derive(Debug, PartialEq)]
pub struct BPMNCollaboration {
    pub name: String,
    pub participants: Vec<BPMNProcess>,
}


impl BPMNCollaboration {
    pub fn add_participant(&mut self, participant: BPMNProcess) {
        self.participants.push(participant);
    }

    pub fn explore_state_space(&self, start_state: State, properties: Vec<GeneralProperty>) -> ModelCheckingResult {
        let mut property_results = vec![];
        let mut not_executed_activities = self.get_all_activities();

        let mut seen_state_hashes = HashMap::new();
        let start_state_hash = calculate_hash(&start_state);
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
                    let potentially_unexplored_states = explore_state(self, &current_state, &mut not_executed_activities);

                    // Check if we know the state already
                    let mut potentially_unexplored_states_hashes = vec![];
                    for new_state in potentially_unexplored_states {
                        let new_hash = calculate_hash(&new_state);
                        match seen_state_hashes.get(&new_hash) {
                            None => {
                                // State is new.
                                seen_state_hashes.insert(new_hash, true);
                                unexplored_states.push((new_hash, new_state));
                            }
                            Some(_) => {}
                        }
                        potentially_unexplored_states_hashes.push(new_hash);
                    }
                    // Do stuff for model checking
                    check_properties(current_state_hash, &current_state, &properties, &mut property_results, &potentially_unexplored_states_hashes);
                    add_terminated_state_hash_if_needed(current_state_hash, &current_state, &mut state_space);

                    // Save the state and its transitions.
                    state_space.states.insert(current_state_hash, current_state);
                    state_space.transitions.insert(current_state_hash, potentially_unexplored_states_hashes);
                }
            };
        }
        determine_properties(&properties, &mut property_results, not_executed_activities);

        ModelCheckingResult {
            state_space,
            property_results,
        }
    }

    fn get_all_activities(&self) -> HashMap<String, bool> {
        let mut never_executed_activities = HashMap::new();
        self.participants.iter().for_each(|process| {
            process.flow_nodes.iter().for_each(|flow_node| {
                if flow_node.flow_node_type == FlowNodeType::Task {
                    // Cloned id here. Could use RC smart pointer instead.
                    never_executed_activities.insert(flow_node.id.clone(), true);
                }
            })
        });
        never_executed_activities
    }

    pub fn create_start_state(&self) -> State {
        let mut start = State {
            snapshots: vec![],
        };
        for process in &self.participants {
            let mut snapshot = ProcessSnapshot {
                // Cloning the string here could be done differently.
                id: process.id.clone(),
                tokens: HashMap::new(),
            };
            for flow_node in &process.flow_nodes {
                match flow_node.flow_node_type {
                    FlowNodeType::StartEvent => {
                        for out_sf in flow_node.outgoing_flows.iter() {
                            // Cloning the string here could be done differently.
                            let position = out_sf.id.clone();
                            snapshot.add_token(position);
                        }
                    }
                    _ => {}
                }
            }
            start.snapshots.push(snapshot);
        }
        start
    }
}

fn add_terminated_state_hash_if_needed(state_hash: u64, state: &State, state_space: &mut StateSpace) {
    if state.is_terminated() {
        state_space.terminated_state_hashes.push(state_hash);
    }
}

fn determine_properties(properties: &Vec<GeneralProperty>, results: &mut Vec<GeneralPropertyResult>, never_executed_activities: HashMap<String, bool>) {
    if properties.contains(&GeneralProperty::NoDeadActivities) {
        // Cannot do this in the loop due to the borrow checker.
        let mut dead_activities: Vec<String> = never_executed_activities.into_keys().collect();
        if !dead_activities.is_empty() {
            dead_activities.sort();
            results.push(GeneralPropertyResult {
                property: GeneralProperty::NoDeadActivities,
                fulfilled: false,
                problematic_elements: dead_activities,
                problematic_state_hashes: vec![],
            });
        } else {
            results.push(GeneralPropertyResult::no_dead_activities());
        }
    }

    for property in properties.iter() {
        match results.iter().find(|result| {
            &result.property == property
        }) {
            None => {
                match property {
                    GeneralProperty::OptionToComplete => {
                        results.push(GeneralPropertyResult::always_terminates())
                    }
                    GeneralProperty::Safeness => {
                        results.push(GeneralPropertyResult::safe())
                    }
                    _ => {}
                }
            }
            Some(_) => {}
        };
    }
}

fn check_properties(current_state_hash: u64,
                    state: &State,
                    properties: &Vec<GeneralProperty>,
                    results: &mut Vec<GeneralPropertyResult>,
                    next_state_hashes: &Vec<u64>) {
    for property in properties.iter() {
        match property {
            GeneralProperty::Safeness => {
                check_if_unsafe(state, results);
            }
            GeneralProperty::OptionToComplete => {
                check_if_stuck(current_state_hash, state, results, next_state_hashes)
            }
            _ => {}
        }
    }
}

fn check_if_stuck(current_state_hash: u64,
                  state: &State,
                  results: &mut Vec<GeneralPropertyResult>,
                  next_state_hashes: &Vec<u64>) {
    if next_state_hashes.is_empty() && !state.is_terminated() {
        match results.iter_mut().find(|result| { result.property == GeneralProperty::OptionToComplete }) {
            None => {
                results.push(GeneralPropertyResult {
                    property: GeneralProperty::OptionToComplete,
                    fulfilled: false,
                    problematic_elements: vec![],
                    problematic_state_hashes: vec![current_state_hash],
                })
            }
            Some(result) => {
                result.problematic_state_hashes.push(current_state_hash)
            }
        }
    }
}

fn check_if_unsafe(state: &State, results: &mut Vec<GeneralPropertyResult>) {
    let two: i16 = 2;
    for snapshot in &state.snapshots {
        match snapshot.tokens.iter().find(|(_, amount)| {
            *amount >= &two
        }) {
            None => {}
            Some((unsafe_flow_element, _)) => {
                results.push(GeneralPropertyResult {
                    property: GeneralProperty::Safeness,
                    fulfilled: false,
                    problematic_elements: vec![unsafe_flow_element.clone()],
                    problematic_state_hashes: vec![],
                })
            }
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn explore_state(collab: &BPMNCollaboration, state: &State, not_executed_activities: &mut HashMap<String, bool>) -> Vec<State> {
    let mut unexplored_states: Vec<State> = vec![];
    for snapshot in &state.snapshots {
        // Find participant for snapshot, could also be hashmap but usually not a long list.
        let option = collab.participants.iter().find(|process_snapshot| { process_snapshot.id == snapshot.id });
        match option {
            None => { panic!("No process found for snapshot with id \"{}\"", snapshot.id) }
            Some(matching_process) => {
                for flow_node in matching_process.flow_nodes.iter() {
                    let mut new_states = flow_node.try_execute(snapshot, state);

                    // Record activity execution
                    if flow_node.flow_node_type == FlowNodeType::Task
                        && new_states.len() > 0
                        && not_executed_activities.len() > 0 {
                        not_executed_activities.remove(&flow_node.id);
                    }

                    // Would want to check if the state has been explored here not later to not take up unnecessary memory.
                    unexplored_states.append(&mut new_states);
                }
            }
        }
    }
    unexplored_states
}

#[derive(Debug, PartialEq)]
pub struct BPMNProcess {
    pub id: String,
    pub flow_nodes: Vec<FlowNode>,
}

impl BPMNProcess {
    pub fn add_sf(&mut self, sf: SequenceFlow, source_ref: String, target_ref: String) {
        let mut source_flow_node: Vec<&mut FlowNode> = self.flow_nodes.iter_mut().filter(|f| f.id == source_ref).collect();
        // TODO: Clone for now but maybe refactor using lifetimes?
        let sf_id = sf.id.clone();

        match source_flow_node.last_mut() {
            None => { panic!("There should be a flow node for the id \"{}\"", source_ref) }
            Some(source) => {
                source.add_outgoing_flow(sf)
            }
        }
        let mut target_flow_node: Vec<&mut FlowNode> = self.flow_nodes.iter_mut().filter(|f| f.id == target_ref).collect();
        match target_flow_node.last_mut() {
            None => { panic!("There should be a flow node for the id \"{}\"", target_ref) }
            Some(target) => { target.add_incoming_flow(SequenceFlow { id: sf_id }) }
        }
    }
    pub fn add_flow_node(&mut self, flow_node: FlowNode) {
        self.flow_nodes.push(flow_node);
    }
}

#[derive(Debug, PartialEq)]
pub struct SequenceFlow {
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub struct FlowNode {
    pub id: String,
    pub flow_node_type: FlowNodeType,
    pub incoming_flows: Vec<SequenceFlow>,
    pub outgoing_flows: Vec<SequenceFlow>,
}

impl FlowNode {
    pub fn add_outgoing_flow(&mut self, sf: SequenceFlow) {
        self.outgoing_flows.push(sf);
    }
    pub fn add_incoming_flow(&mut self, sf: SequenceFlow) {
        self.incoming_flows.push(sf);
    }
    pub fn new(id: String, flow_node_type: FlowNodeType) -> FlowNode {
        FlowNode { id, flow_node_type, incoming_flows: Vec::new(), outgoing_flows: Vec::new() }
    }
    pub fn try_execute(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        match self.flow_node_type {
            FlowNodeType::StartEvent => { vec![] }
            FlowNodeType::Task => { self.try_execute_task(snapshot, current_state) }
            FlowNodeType::IntermediateThrowEvent => { self.try_intermediate_throw_event(snapshot, current_state) }
            FlowNodeType::ExclusiveGateway => { self.try_execute_exg(snapshot, current_state) }
            FlowNodeType::ParallelGateway => { self.try_execute_pg(snapshot, current_state) }
            FlowNodeType::EndEvent => { self.try_execute_end_event(snapshot, current_state) }
        }
    }

    fn try_execute_pg(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        if self.missing_token_for_pg(snapshot) {
            return vec![];
        }
        // Clone all snapshots and tokens
        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
        let mut new_snapshot = ProcessSnapshot {
            id: snapshot.id.clone(),
            // Remove incoming tokens
            tokens: snapshot.tokens.clone(),
        };
        // Remove incoming tokens
        for in_sf in self.incoming_flows.iter() {
            new_snapshot.remove_token(in_sf.id.clone());
        }
        // Add outgoing tokens
        self.add_outgoing_tokens(&mut new_snapshot);
        new_state.snapshots.push(new_snapshot);
        vec![new_state]
    }

    fn missing_token_for_pg(&self, snapshot: &ProcessSnapshot) -> bool {
        !self.incoming_flows.iter().all(|sf| { snapshot.tokens.contains_key(&sf.id) })
    }

    fn create_new_state_without_snapshot(snapshot: &ProcessSnapshot, current_state: &State) -> State {
        // Clone should be avoided.
        let mut snapshots = current_state.snapshots.clone();
        // Remove the snapshot
        let index = snapshots.iter()
            .position(|sp| { snapshot.id == sp.id })
            .expect("Snapshot not found!");
        snapshots.swap_remove(index);

        State {
            snapshots,
        }
    }

    fn add_outgoing_tokens(&self, snapshot: &mut ProcessSnapshot) {
        for out_flow in self.outgoing_flows.iter() {
            snapshot.add_token(out_flow.id.clone());
        }
    }
    fn try_execute_task(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Add new state
                    let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
                    let mut new_snapshot = Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);
                    // Add outgoing tokens
                    self.add_outgoing_tokens(&mut new_snapshot);
                    new_state.snapshots.push(new_snapshot);

                    new_states.push(new_state);
                }
            }
        }
        new_states
    }
    fn try_intermediate_throw_event(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        // Currently the same as task but event types will change this.
        self.try_execute_task(snapshot, current_state)
    }
    fn try_execute_exg(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Add one state with a token for each outgoing flow
                    for out_flow in self.outgoing_flows.iter() {
                        // Add new state
                        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
                        let mut new_snapshot = Self::create_new_snapshot_without_token(snapshot, &inc_flow.id.clone());
                        // Add outgoing token
                        new_snapshot.add_token(out_flow.id.clone());
                        new_state.snapshots.push(new_snapshot);

                        new_states.push(new_state);
                    }
                }
            }
        }
        new_states
    }

    fn create_new_snapshot_without_token(snapshot: &ProcessSnapshot, token: &String) -> ProcessSnapshot {
        let mut snapshot = ProcessSnapshot {
            id: snapshot.id.clone(),
            // Remove incoming token
            tokens: snapshot.tokens.clone(),
        };
        snapshot.remove_token(token.clone());
        snapshot
    }
    fn try_execute_end_event(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Consume incoming token
                    let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
                    let new_snapshot = Self::create_new_snapshot_without_token(snapshot, &inc_flow.id.clone());
                    // Add outgoing token
                    new_state.snapshots.push(new_snapshot);

                    new_states.push(new_state);
                }
            }
        }
        new_states
    }
}

#[derive(Debug, PartialEq)]
pub enum FlowNodeType {
    StartEvent,
    IntermediateThrowEvent,
    Task,
    ExclusiveGateway,
    ParallelGateway,
    EndEvent,
}