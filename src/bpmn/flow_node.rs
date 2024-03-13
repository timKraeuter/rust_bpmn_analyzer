use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::process::Process;
use crate::states::state_space::{ProcessSnapshot, State};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct SequenceFlow {
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub struct MessageFlow {
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub struct FlowNode {
    pub id: String,
    pub flow_node_type: FlowNodeType,
    pub incoming_flows: Vec<SequenceFlow>,
    pub outgoing_flows: Vec<SequenceFlow>,
    pub incoming_message_flows: Vec<MessageFlow>,
    pub outgoing_message_flows: Vec<MessageFlow>,
}

impl FlowNode {
    pub fn new(id: String, flow_node_type: FlowNodeType) -> FlowNode {
        FlowNode {
            id,
            flow_node_type,
            incoming_flows: vec![],
            outgoing_flows: vec![],
            incoming_message_flows: vec![],
            outgoing_message_flows: vec![],
        }
    }
    pub fn add_outgoing_flow(&mut self, sf: SequenceFlow) {
        self.outgoing_flows.push(sf);
    }
    pub fn add_incoming_flow(&mut self, sf: SequenceFlow) {
        self.incoming_flows.push(sf);
    }
    pub fn add_outgoing_message_flow(&mut self, mf: MessageFlow) {
        self.outgoing_message_flows.push(mf);
    }
    pub fn add_incoming_message_flow(&mut self, mf: MessageFlow) {
        self.incoming_message_flows.push(mf);
    }
    pub fn try_execute(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
        collaboration: &Collaboration,
    ) -> Vec<State> {
        match &self.flow_node_type {
            FlowNodeType::StartEvent(_) => vec![],
            FlowNodeType::Task(_) => self.try_execute_task(snapshot, current_state),
            FlowNodeType::IntermediateThrowEvent(_) => {
                self.try_execute_intermediate_throw_event(snapshot, current_state)
            }
            FlowNodeType::ExclusiveGateway => self.try_execute_exg(snapshot, current_state),
            FlowNodeType::ParallelGateway => self.try_execute_pg(snapshot, current_state),
            FlowNodeType::EventBasedGateway => {
                self.try_execute_evg(snapshot, current_state, collaboration)
            }
            FlowNodeType::EndEvent(e) => self.try_execute_end_event(snapshot, current_state, e),
            FlowNodeType::IntermediateCatchEvent(_) => {
                self.try_execute_intermediate_catch_event(snapshot, current_state)
            }
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
            tokens: snapshot.tokens.clone(),
        };
        // Remove incoming tokens
        for in_sf in self.incoming_flows.iter() {
            new_snapshot.delete_token(&in_sf.id);
        }
        // Add outgoing tokens
        self.add_outgoing_tokens(&mut new_snapshot);
        new_state.snapshots.push(new_snapshot);
        vec![new_state]
    }

    fn missing_token_for_pg(&self, snapshot: &ProcessSnapshot) -> bool {
        !self
            .incoming_flows
            .iter()
            .all(|sf| snapshot.tokens.contains_key(&sf.id))
    }

    fn create_new_state_without_snapshot(
        snapshot: &ProcessSnapshot,
        current_state: &State,
    ) -> State {
        // Clone all but the defined one.
        let snapshots = current_state
            .snapshots
            .iter()
            .filter_map(|sp| {
                if sp.id == snapshot.id {
                    None
                } else {
                    Some(sp.clone())
                }
            })
            .collect();

        State {
            snapshots,
            executed_end_event_counter: current_state.executed_end_event_counter.clone(),
            messages: BTreeMap::new(),
        }
    }

    fn add_outgoing_tokens(&self, snapshot: &mut ProcessSnapshot) {
        for out_flow in self.outgoing_flows.iter() {
            snapshot.add_token(&out_flow.id);
        }
    }
    fn try_execute_task(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.

        if self.flow_node_type == FlowNodeType::Task(TaskType::Receive)
            && self.no_message_flow_has_a_message(current_state)
        {
            return vec![];
        }
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Add new state
                    let mut new_state =
                        Self::create_new_state_without_snapshot(snapshot, current_state);
                    let mut new_snapshot =
                        Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);

                    self.add_outgoing_tokens(&mut new_snapshot);
                    new_state.snapshots.push(new_snapshot);

                    self.add_outgoing_messages(&mut new_state);

                    new_states.push(new_state);
                }
            }
        }
        new_states
    }

    fn add_outgoing_messages(&self, new_state: &mut State) {
        for out_mf in self.outgoing_message_flows.iter() {
            new_state.add_message(&out_mf.id);
        }
    }
    fn try_execute_intermediate_throw_event(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
    ) -> Vec<State> {
        // Currently the same as task but event types will change this.
        // Still fine since it creates messages just like tasks.
        self.try_execute_task(snapshot, current_state)
    }
    fn try_execute_exg(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = vec![]; // Could set capacity to number of outgoing flows.
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Add one state with a token for each outgoing flow
                    for out_flow in self.outgoing_flows.iter() {
                        // Add new state
                        let mut new_state =
                            Self::create_new_state_without_snapshot(snapshot, current_state);
                        let mut new_snapshot =
                            Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);
                        // Add outgoing token
                        new_snapshot.add_token(&out_flow.id);
                        new_state.snapshots.push(new_snapshot);

                        new_states.push(new_state);
                    }
                }
            }
        }
        new_states
    }

    fn create_new_snapshot_without_token(
        snapshot: &ProcessSnapshot,
        token: &str,
    ) -> ProcessSnapshot {
        let mut snapshot = ProcessSnapshot {
            id: snapshot.id.clone(),
            // Remove incoming token
            tokens: snapshot.tokens.clone(),
        };
        snapshot.delete_token(token);
        snapshot
    }
    fn try_execute_end_event(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
        event_type: &EventType,
    ) -> Vec<State> {
        let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    if event_type == &EventType::Terminate {
                        return self.execute_terminate_end_event(snapshot, current_state);
                    }

                    // Consume incoming token
                    let mut new_state =
                        Self::create_new_state_without_snapshot(snapshot, current_state);
                    let new_snapshot =
                        Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);
                    new_state.snapshots.push(new_snapshot);
                    self.record_end_event_execution(&mut new_state);

                    self.add_outgoing_messages(&mut new_state);

                    new_states.push(new_state);
                }
            }
        }
        new_states
    }

    fn execute_terminate_end_event(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
    ) -> Vec<State> {
        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
        let new_snapshot = ProcessSnapshot {
            id: snapshot.id.clone(),
            tokens: BTreeMap::new(), // All tokens are removed due to terminate.
        };
        new_state.snapshots.push(new_snapshot);
        self.record_end_event_execution(&mut new_state);
        vec![new_state]
    }

    fn try_execute_intermediate_catch_event(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
    ) -> Vec<State> {
        match self.flow_node_type {
            FlowNodeType::IntermediateCatchEvent(EventType::Message) => {
                let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.
                if self.no_message_flow_has_a_message(current_state) {
                    return vec![];
                }
                for inc_flow in self.incoming_flows.iter() {
                    match snapshot.tokens.get(&inc_flow.id) {
                        None => {}
                        Some(_) => {
                            // Consume incoming token
                            let mut new_state =
                                Self::create_new_state_without_snapshot(snapshot, current_state);
                            let mut new_snapshot =
                                Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);
                            // Add outgoing tokens
                            self.add_outgoing_tokens(&mut new_snapshot);

                            new_state.snapshots.push(new_snapshot);

                            new_states.push(new_state);
                        }
                    }
                }
                new_states
            }
            FlowNodeType::IntermediateCatchEvent(EventType::None) => {
                self.try_execute_task(snapshot, current_state)
            }
            _ => {
                vec![]
            }
        }
    }

    fn no_message_flow_has_a_message(&self, current_state: &State) -> bool {
        !self
            .incoming_message_flows
            .iter()
            .any(|inc_mf| current_state.messages.get(&inc_mf.id).is_some())
    }

    fn record_end_event_execution(&self, new_state: &mut State) {
        match new_state.executed_end_event_counter.get(&self.id) {
            None => new_state
                .executed_end_event_counter
                .insert(self.id.clone(), 1),
            Some(count) => new_state
                .executed_end_event_counter
                .insert(self.id.clone(), count + 1),
        };
    }
    pub fn try_trigger_message_start_event(
        &self,
        process: &Process,
        current_state: &State,
    ) -> Vec<State> {
        let mut next_states = vec![];
        if current_state.messages.is_empty() {
            return next_states;
        }
        for inc_mf in self.incoming_message_flows.iter() {
            let message_count = current_state.messages.get(&inc_mf.id);
            match message_count {
                None => {}
                Some(count) => {
                    if *count > 0 {
                        let mut new_state = State {
                            snapshots: current_state.snapshots.clone(),
                            executed_end_event_counter: current_state
                                .executed_end_event_counter
                                .clone(),
                            messages: BTreeMap::new(),
                        };
                        // Create a new snapshot.
                        let mut new_snapshot = ProcessSnapshot {
                            id: process.id.clone(),
                            tokens: BTreeMap::new(),
                        };
                        // Add outgoing tokens
                        self.add_outgoing_tokens(&mut new_snapshot);
                        new_state.snapshots.push(new_snapshot);
                        next_states.push(new_state);
                    }
                }
            }
        }
        next_states
    }
    fn try_execute_evg(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
        collaboration: &Collaboration,
    ) -> Vec<State> {
        // Currently only messages can trigger evgs.
        if current_state.messages.is_empty() {
            return vec![];
        }
        let mut new_states: Vec<State> = Vec::with_capacity(1);
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Find next flow nodes after the EVG. Currently very annoying! to be improved.
                    let next_flow_nodes = collaboration
                        .participants
                        .iter()
                        .filter(|p| p.id == snapshot.id)
                        .flat_map(|p| p.flow_nodes.iter())
                        .filter(|f| {
                            !f.incoming_message_flows.is_empty()
                                && (f.flow_node_type
                                    == FlowNodeType::IntermediateCatchEvent(EventType::Message)
                                    || f.flow_node_type == FlowNodeType::Task(TaskType::Receive))
                                && f.incoming_flows
                                    .iter()
                                    .any(|sf| self.outgoing_flows.iter().any(|of| sf.id == of.id))
                        })
                        .collect::<Vec<&FlowNode>>();
                    // Add outgoing tokens of the triggered event/receive task after the gateway.
                    for flow_node in next_flow_nodes.iter() {
                        if flow_node.no_message_flow_has_a_message(current_state) {
                            continue;
                        }
                        // Consume incoming token
                        let mut new_state =
                            Self::create_new_state_without_snapshot(snapshot, current_state);
                        let mut new_snapshot =
                            Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);

                        // Add outgoing tokens
                        flow_node.add_outgoing_tokens(&mut new_snapshot);

                        new_state.snapshots.push(new_snapshot);

                        new_states.push(new_state);
                    }
                }
            }
        }
        new_states
    }
}

#[derive(Debug, PartialEq)]
pub enum FlowNodeType {
    StartEvent(EventType),
    IntermediateThrowEvent(EventType),
    IntermediateCatchEvent(EventType),
    Task(TaskType),
    ExclusiveGateway,
    ParallelGateway,
    EventBasedGateway,
    EndEvent(EventType),
}

#[derive(Debug, PartialEq)]
pub enum EventType {
    None,
    Message,
    Unsupported,
    Terminate,
}

#[derive(Debug, PartialEq)]
pub enum TaskType {
    Default,
    Receive,
}
