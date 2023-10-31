use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub use reader::read_bpmn_file;
pub use state_space::StateSpace;

use crate::bpmn::state_space::{ProcessSnapshot, State, Token};

mod reader;
mod state_space;

#[derive(Debug, PartialEq)]
pub struct BPMNCollaboration {
    pub name: String,
    pub participants: Vec<BPMNProcess>,
}

impl BPMNCollaboration {
    pub fn add_participant(&mut self, participant: BPMNProcess) {
        self.participants.push(participant);
    }

    pub fn explore_state_space(&self, start_state: State) -> StateSpace {
        let mut state_hashes = vec![];

        let mut explored_states = vec![];
        let mut unexplored_states = vec![start_state];
        while !unexplored_states.is_empty() {
            match unexplored_states.pop() {
                None => {}
                Some(current_state) => {
                    state_hashes.push(calculate_hash(&current_state));

                    // Explore the state
                    let potentially_unexplored_states = explore_state(self, &current_state);
                    // Check if we know the state already
                    potentially_unexplored_states.into_iter()
                        .filter(|state| {
                            let hash = calculate_hash(state);
                            !state_hashes.iter().any(|state_hash| { *state_hash == hash })
                        })
                        .for_each(|state| {
                            unexplored_states.push(state)
                        });

                    // Explored states are saved.
                    explored_states.push(current_state);
                }
            };
            println!("{} states to be explored", unexplored_states.len())
        }
        println!("{:?}", state_hashes);
        StateSpace {
            states: explored_states
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
                tokens: Vec::new(),
            };
            for flow_node in &process.flow_nodes {
                match flow_node.flow_node_type {
                    // Cloning the string here could be done differently.
                    FlowNodeType::StartEvent => {
                        for out_sf in flow_node.outgoing_flows.iter() {
                            snapshot.tokens.push(Token { position: out_sf.id.clone() })
                        }
                    }
                    FlowNodeType::Task => {}
                    FlowNodeType::ExclusiveGateway => {}
                    FlowNodeType::ParallelGateway => {}
                    FlowNodeType::EndEvent => {}
                }
            }
            start.snapshots.push(snapshot);
        }
        start
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
        for inc_flow in self.incoming_flows.iter() {
            if !snapshot.tokens.iter().any(|token| { token.position == inc_flow.id }) {
                return vec![];
            }
        }
        // Clone all snapshots and tokens
        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
        let mut new_snapshot = ProcessSnapshot {
            id: snapshot.id.clone(),
            // Remove incoming tokens
            tokens: snapshot.tokens.iter().filter(|t| { !self.incoming_flows.iter().any(|inc_sf| { inc_sf.id == t.position }) }).cloned().collect(),
        };
        // Add outgoing tokens
        self.add_outgoing_tokens(&mut new_snapshot);
        new_state.snapshots.push(new_snapshot);
        vec![new_state]
    }

    fn create_new_state_without_snapshot(snapshot: &ProcessSnapshot, current_state: &State) -> State {
        State {
            // Do not copy snapshot but the rest.
            snapshots: current_state.snapshots.iter().filter(|x| { x.id != snapshot.id }).cloned().collect()
        }
    }

    fn add_outgoing_tokens(&self, new_snapshot: &mut ProcessSnapshot) {
        for out_flow in self.outgoing_flows.iter() {
            new_snapshot.tokens.push(Token {
                position: out_flow.id.clone()
            })
        }
    }
    fn try_execute_task(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            let token_at_flow = snapshot.tokens.iter().find(|token| { token.position == inc_flow.id });
            match token_at_flow {
                None => {}
                Some(token_at_flow) => {
                    // Add new state
                    let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
                    let mut new_snapshot = Self::create_new_snapshot_without_token(snapshot, token_at_flow);
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
            let token_at_flow = snapshot.tokens.iter().find(|token| { token.position == inc_flow.id });
            match token_at_flow {
                None => {}
                Some(token_at_flow) => {
                    // Add one token for each outgoing flow
                    for out_flow in self.outgoing_flows.iter() {
                        // Add new state
                        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
                        let mut new_snapshot = Self::create_new_snapshot_without_token(snapshot, token_at_flow);
                        // Add outgoing token
                        new_snapshot.tokens.push(Token {
                            position: out_flow.id.clone()
                        });
                        new_state.snapshots.push(new_snapshot);

                        new_states.push(new_state);
                    }
                }
            }
        }
        new_states
    }

    fn create_new_snapshot_without_token(snapshot: &ProcessSnapshot, token: &Token) -> ProcessSnapshot {
        ProcessSnapshot {
            id: snapshot.id.clone(),
            // Remove incoming token
            tokens: snapshot.tokens.iter().filter(|t| { t.position != token.position }).cloned().collect(),
        }
    }
    fn try_execute_end_event(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            let token_at_flow = snapshot.tokens.iter().find(|token| { token.position == inc_flow.id });
            match token_at_flow {
                None => {}
                Some(token_at_flow) => {
                    // Consume incoming token
                    let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
                    let new_snapshot = Self::create_new_snapshot_without_token(snapshot, token_at_flow);
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