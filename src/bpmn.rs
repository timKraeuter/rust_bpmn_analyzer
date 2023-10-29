mod reader;
mod state_space;

pub use reader::read_bpmn_file;
pub use state_space::StateSpace;
use crate::bpmn::state_space::{ProcessSnapshot, State, Token};

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
        let mut explored_states = Vec::new();
        let mut unexplored_states = vec![start_state];
        while !unexplored_states.is_empty() {
            match unexplored_states.pop() {
                None => {}
                Some(state) => {
                    // Explore the state
                    explore_state(self, &state, &mut unexplored_states);

                    // Explored states are saved.
                    explored_states.push(state);
                }
            };
            // TODO: Remove later
            if explored_states.len() == 5 { break; }
        }
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
                    FlowNodeType::StartEvent => { snapshot.tokens.push(Token { position: flow_node.id.clone() }) }
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

fn explore_state(collab: &BPMNCollaboration, state: &State, unexplored_states: &mut Vec<State>) {
    for snapshot in &state.snapshots {
        // Find participant for snapshot
        let option = collab.participants.iter().find(|process_snapshot| { process_snapshot.id == snapshot.id });
        match option {
            None => { panic!("No process found for snapshot with id \"{}\"", snapshot.id) }
            Some(matching_process) => {
                for token in &snapshot.tokens {
                    // Find flow node(s) the token is sitting in front.
                    matching_process.flow_nodes.iter()
                        .flat_map(|x| { &x.incoming_flows })
                        .find(|inc_flow| { inc_flow.id == token.position });
                    // Result should be one max.

                    // Check if that flow node can be executed

                    // Add create new state and add to the unexplored states.
                }
            }
        }
    }


    // Add all new states to the unexplored states.
    // let new_state = State {
    //     // Cloning snapshots also not optimal.
    //     snapshots: state.snapshots.to_vec()
    // };
    // unexplored_states.push(new_state);
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
}

#[derive(Debug, PartialEq)]
pub enum FlowNodeType {
    StartEvent,
    Task,
    ExclusiveGateway,
    ParallelGateway,
    EndEvent,
}