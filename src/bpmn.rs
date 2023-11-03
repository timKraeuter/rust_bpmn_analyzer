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
        let mut state_hashes = HashMap::new();
        state_hashes.insert(calculate_hash(&start_state), true);

        let mut explored_states = vec![];
        let mut unexplored_states = vec![start_state];
        while !unexplored_states.is_empty() {
            match unexplored_states.pop() {
                None => {}
                Some(current_state) => {
                    // Explore the state
                    let potentially_unexplored_states = explore_state(self, &current_state);
                    // Check if we know the state already
                    for state in potentially_unexplored_states {
                        Self::record_unexplored_state(&mut state_hashes, &mut unexplored_states, state);
                    }

                    // Some properties are checked for each state.
                    check_properties(&current_state, &properties, &mut property_results);
                    // Explored states are saved.
                    explored_states.push(current_state);
                }
            };
            // println!("{} states to be explored", unexplored_states.len())
        }
        ModelCheckingResult {
            state_space: StateSpace {
                states: explored_states
            },
            property_results
        }

    }

    fn record_unexplored_state(
        state_hashes: &mut HashMap<u64, bool>,
        unexplored_states: &mut Vec<State>,
        potentially_new_state: State) {
        let hash = calculate_hash(&potentially_new_state);
        match state_hashes.get(&hash) {
            None => {
                // State is new.
                state_hashes.insert(hash, true);
                unexplored_states.push(potentially_new_state)
            }
            Some(_) => {}
        }
    }

    pub fn create_start_state(&self) -> State {
        let mut start = State {
            snapshots: Vec::new()
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

fn check_properties(state: &State, properties: &Vec<GeneralProperty>, results: &mut Vec<GeneralPropertyResult>) {
    for property in properties.iter() {
        match property {
            GeneralProperty::Safeness => {
                check_if_unsafe(state, results);
            }
            _ => {}
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
                    problematic_elements: vec![unsafe_flow_element.clone()]
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

fn explore_state(collab: &BPMNCollaboration, state: &State) -> Vec<State> {
    let mut unexplored_states: Vec<State> = vec![];
    for snapshot in &state.snapshots {
        // Find participant for snapshot
        let option = collab.participants.iter().find(|process_snapshot| { process_snapshot.id == snapshot.id });
        match option {
            None => { panic!("No process found for snapshot with id \"{}\"", snapshot.id) }
            Some(matching_process) => {
                for flow_node in matching_process.flow_nodes.iter() {
                    let mut new_states = flow_node.try_execute(snapshot, state);
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
            snapshots
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
    fn try_execute_exg(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Add one token for each outgoing flow
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
    Task,
    ExclusiveGateway,
    ParallelGateway,
    EndEvent,
}