use crate::bpmn::flow_node::EventType::Link;
use crate::bpmn::process::Process;
use crate::states::state_space::{ProcessSnapshot, State};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq)]
pub struct SequenceFlow {
    pub id: String,
    pub target_idx: usize,
    pub source_idx: usize,
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
    pub fn try_execute<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
        process: &'a Process,
        not_executed_activities: &mut HashMap<&str, bool>,
    ) -> Vec<State<'a>> {
        match &self.flow_node_type {
            FlowNodeType::StartEvent(_) => vec![],
            FlowNodeType::Task(_) => self.try_execute_task(snapshot, current_state),
            FlowNodeType::IntermediateThrowEvent(_) => {
                self.try_execute_intermediate_throw_event(snapshot, current_state, process)
            }
            FlowNodeType::ExclusiveGateway => self.try_execute_exg(snapshot, current_state),
            FlowNodeType::ParallelGateway => self.try_execute_pg(snapshot, current_state),
            FlowNodeType::EventBasedGateway => {
                self.try_execute_evg(snapshot, current_state, process, not_executed_activities)
            }
            FlowNodeType::EndEvent(e) => self.try_execute_end_event(snapshot, current_state, e),
            FlowNodeType::IntermediateCatchEvent(_) => {
                self.try_execute_intermediate_catch_event(snapshot, current_state)
            }
        }
    }

    fn try_execute_pg<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> Vec<State<'a>> {
        if self.missing_token_for_pg(snapshot) {
            return vec![];
        }
        // Clone all snapshots and tokens
        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
        let mut new_snapshot = ProcessSnapshot {
            id: snapshot.id,
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
            .all(|sf| snapshot.tokens.contains_key(sf.id.as_str()))
    }

    fn create_new_state_without_snapshot<'a, 'b>(
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> State<'a> {
        let snapshots = Self::clone_snapshots_without_snapshot(snapshot, current_state);

        State {
            snapshots,
            executed_end_event_counter: current_state.executed_end_event_counter.clone(),
            messages: current_state.messages.clone(),
        }
    }

    fn create_new_state_without_snapshot_and_message<'a, 'b>(
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
        message_id: &str,
    ) -> State<'a> {
        let snapshots = Self::clone_snapshots_without_snapshot(snapshot, current_state);

        State {
            snapshots,
            executed_end_event_counter: current_state.executed_end_event_counter.clone(),
            messages: Self::clone_decrease_message(message_id, current_state),
        }
    }

    fn clone_snapshots_without_snapshot<'a, 'b>(
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> Vec<ProcessSnapshot<'a>> {
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
        snapshots
    }

    fn add_outgoing_tokens<'a>(&'a self, snapshot: &mut ProcessSnapshot<'a>) {
        for out_flow in self.outgoing_flows.iter() {
            snapshot.add_token(&out_flow.id);
        }
    }
    fn try_execute_task<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> Vec<State<'a>> {
        // Usually there is only one incoming flow, i.e., only one new state.
        let mut new_states: Vec<State> = Vec::with_capacity(1);

        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(inc_flow.id.as_str()) {
                None => {}
                Some(_) => {
                    if self.flow_node_type == FlowNodeType::Task(TaskType::Receive) {
                        // Handle message task
                        let messages = self.get_message_flows_with_message(current_state);
                        if messages.is_empty() {
                            continue;
                        }
                        for message in messages {
                            let mut new_state = Self::create_new_state_without_snapshot_and_message(
                                snapshot,
                                current_state,
                                message,
                            );
                            let mut new_snapshot =
                                Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);

                            self.add_outgoing_tokens(&mut new_snapshot);
                            new_state.snapshots.push(new_snapshot);

                            self.add_outgoing_messages(&mut new_state);

                            new_states.push(new_state);
                        }
                    } else {
                        // Handle normal task
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
        }
        new_states
    }

    fn add_outgoing_messages<'a>(&'a self, new_state: &mut State<'a>) {
        for out_mf in self.outgoing_message_flows.iter() {
            new_state.add_message(&out_mf.id);
        }
    }

    fn try_execute_intermediate_throw_event<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
        process: &'a Process,
    ) -> Vec<State<'a>> {
        match &self.flow_node_type {
            FlowNodeType::IntermediateThrowEvent(Link(link_name)) => {
                self.try_execute_link_throw_event(snapshot, current_state, link_name, process)
            }
            _ => self.try_execute_task(snapshot, current_state),
        }
    }

    fn try_execute_link_throw_event<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
        link_name: &str,
        process: &'a Process,
    ) -> Vec<State<'a>> {
        let mut new_states = vec![];
        let matching_link_catch_event = self.find_matching_link_catch_event(process, link_name);
        match matching_link_catch_event {
            None => {}
            Some(matching_link_catch_event) => {
                for inc_flow in self.incoming_flows.iter() {
                    match snapshot.tokens.get(inc_flow.id.as_str()) {
                        None => {}
                        Some(_) => {
                            let mut new_state =
                                Self::create_new_state_without_snapshot(snapshot, current_state);
                            let mut new_snapshot =
                                Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);

                            matching_link_catch_event.add_outgoing_tokens(&mut new_snapshot);
                            new_state.snapshots.push(new_snapshot);
                            new_states.push(new_state);
                        }
                    };
                }
            }
        }
        new_states
    }

    fn find_matching_link_catch_event<'a>(
        &'a self,
        process: &'a Process,
        link_name: &str,
    ) -> Option<&'a FlowNode> {
        process
            .flow_nodes
            .iter()
            .find(|flow_node| match &flow_node.flow_node_type {
                FlowNodeType::IntermediateCatchEvent(Link(other_link_name)) => {
                    other_link_name == link_name
                }
                _ => false,
            })
    }

    fn try_execute_exg<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> Vec<State<'a>> {
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(inc_flow.id.as_str()) {
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

    fn create_new_snapshot_without_token<'a>(
        snapshot: &ProcessSnapshot<'a>,
        token: &str,
    ) -> ProcessSnapshot<'a> {
        let mut snapshot = ProcessSnapshot {
            id: snapshot.id,
            // Remove incoming token
            tokens: snapshot.tokens.clone(),
        };
        snapshot.delete_token(token);
        snapshot
    }

    fn try_execute_end_event<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
        event_type: &'a EventType,
    ) -> Vec<State<'a>> {
        let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(inc_flow.id.as_str()) {
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

    fn execute_terminate_end_event<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> Vec<State<'a>> {
        let mut new_state = Self::create_new_state_without_snapshot(snapshot, current_state);
        let new_snapshot = ProcessSnapshot {
            id: snapshot.id,
            tokens: BTreeMap::new(), // All tokens are removed due to terminate.
        };
        new_state.snapshots.push(new_snapshot);
        self.record_end_event_execution(&mut new_state);
        vec![new_state]
    }

    fn try_execute_intermediate_catch_event<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
    ) -> Vec<State<'a>> {
        match self.flow_node_type {
            FlowNodeType::IntermediateCatchEvent(EventType::Message) => {
                let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.
                let message_flows_with_messages =
                    self.get_message_flows_with_message(current_state);
                if message_flows_with_messages.is_empty() {
                    return vec![];
                }
                for inc_flow in self.incoming_flows.iter() {
                    match snapshot.tokens.get(inc_flow.id.as_str()) {
                        None => {}
                        Some(_) => {
                            for &message in message_flows_with_messages.iter() {
                                // Consume incoming token and message.
                                let mut new_state =
                                    Self::create_new_state_without_snapshot_and_message(
                                        snapshot,
                                        current_state,
                                        message,
                                    );
                                let mut new_snapshot =
                                    Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);
                                // Add outgoing tokens
                                self.add_outgoing_tokens(&mut new_snapshot);

                                new_state.snapshots.push(new_snapshot);

                                new_states.push(new_state);
                            }
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

    fn get_message_flows_with_message(&self, current_state: &State) -> Vec<&str> {
        self.incoming_message_flows
            .iter()
            .filter_map(|inc_mf| {
                if current_state.messages.get(inc_mf.id.as_str()).is_some() {
                    return Some(inc_mf.id.as_str());
                }
                None
            })
            .collect()
    }

    fn record_end_event_execution<'a>(&'a self, new_state: &mut State<'a>) {
        match new_state.executed_end_event_counter.get(self.id.as_str()) {
            None => new_state.executed_end_event_counter.insert(&self.id, 1),
            Some(count) => new_state
                .executed_end_event_counter
                .insert(&self.id, count + 1),
        };
    }
    pub fn try_trigger_message_start_event<'a>(
        &'a self,
        process: &'a Process,
        current_state: &State<'a>,
    ) -> Vec<State<'a>> {
        let mut next_states = vec![];
        if current_state.messages.is_empty() {
            return next_states;
        }
        for inc_mf in self.incoming_message_flows.iter() {
            let message_id = inc_mf.id.as_str();
            let message_count = current_state.messages.get(message_id);
            match message_count {
                None => {}
                Some(count) => {
                    if *count > 0 {
                        let mut new_state = State {
                            snapshots: current_state.snapshots.clone(),
                            executed_end_event_counter: current_state
                                .executed_end_event_counter
                                .clone(),
                            messages: Self::clone_decrease_message(message_id, current_state),
                        };
                        // Create a new snapshot.
                        let mut new_snapshot = ProcessSnapshot {
                            id: &process.id,
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

    fn clone_decrease_message<'a>(message_id: &str, state: &State<'a>) -> BTreeMap<&'a str, u16> {
        let mut messages = BTreeMap::new();
        state.messages.iter().for_each(|(&message, count)| {
            if message == message_id {
                if *count > 1 {
                    messages.insert(message, *count - 1);
                }
            } else {
                messages.insert(message, *count);
            }
        });
        messages
    }

    fn try_execute_evg<'a, 'b>(
        &'a self,
        snapshot: &'b ProcessSnapshot<'a>,
        current_state: &'b State<'a>,
        process: &'a Process,
        not_executed_activities: &mut HashMap<&str, bool>,
    ) -> Vec<State<'a>> {
        // Currently only messages can trigger evgs.
        if current_state.messages.is_empty() {
            return vec![];
        }
        let mut new_states: Vec<State> = vec![];
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(inc_flow.id.as_str()) {
                None => {}
                Some(_) => {
                    // Add outgoing tokens for the triggered event/receive task after the gateway.
                    for flow_node in self.outgoing_flows.iter().filter_map(|sequence_flow| {
                        process.flow_nodes.get(sequence_flow.target_idx)
                    }) {
                        let message_flows_with_incoming_messages =
                            flow_node.get_message_flows_with_message(current_state);
                        if message_flows_with_incoming_messages.is_empty() {
                            continue;
                        }
                        if flow_node.flow_node_type == FlowNodeType::Task(TaskType::Receive)
                            || flow_node.flow_node_type == FlowNodeType::Task(TaskType::Default)
                        {
                            not_executed_activities.remove(flow_node.id.as_str());
                        }
                        for message in message_flows_with_incoming_messages {
                            // Consume incoming token
                            let mut new_state = Self::create_new_state_without_snapshot_and_message(
                                snapshot,
                                current_state,
                                message,
                            );
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
    Terminate,
    Link(String),
}

#[derive(Debug, PartialEq)]
pub enum TaskType {
    Default,
    Receive,
}
