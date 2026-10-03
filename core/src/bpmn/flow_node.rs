use crate::bpmn::flow_node::EventType::Link;
use crate::bpmn::process::Process;
use crate::model_checking::por::independence::TransitionEffect;
use crate::states::state_space::{ProcessSnapshot, State};
use crate::states::successor::{Instance, StateChange};
use std::collections::HashSet;
use std::slice;

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
        not_executed_activities: &mut HashSet<&str>,
    ) -> Vec<State<'a>> {
        let mut changes = vec![];
        self.collect_state_changes(
            snapshot,
            current_state,
            process,
            not_executed_activities,
            &mut changes,
        );
        changes
            .iter()
            .map(|change| change.apply(current_state, Instance::Existing(snapshot)))
            .collect()
    }

    /// Collects the changes of all possible executions of this flow node by the given snapshot.
    pub(crate) fn collect_state_changes<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        current_state: &State<'a>,
        process: &'a Process,
        not_executed_activities: &mut HashSet<&str>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        match &self.flow_node_type {
            FlowNodeType::StartEvent(_) => {}
            FlowNodeType::Task(_) => self.try_execute_task(snapshot, current_state, changes),
            FlowNodeType::IntermediateThrowEvent(_) => {
                self.try_execute_intermediate_throw_event(snapshot, current_state, process, changes)
            }
            FlowNodeType::ExclusiveGateway => self.try_execute_exg(snapshot, changes),
            FlowNodeType::ParallelGateway => self.try_execute_pg(snapshot, changes),
            FlowNodeType::EventBasedGateway => self.try_execute_evg(
                snapshot,
                current_state,
                process,
                not_executed_activities,
                changes,
            ),
            FlowNodeType::EndEvent(e) => self.try_execute_end_event(snapshot, e, changes),
            FlowNodeType::IntermediateCatchEvent(_) => {
                self.try_execute_intermediate_catch_event(snapshot, current_state, changes)
            }
        }
    }

    fn try_execute_pg<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        if self.missing_token_for_pg(snapshot) {
            return;
        }
        changes.push(StateChange {
            // Remove incoming tokens
            consumes_tokens: &self.incoming_flows,
            // Add outgoing tokens
            produces_tokens: &self.outgoing_flows,
            ..Default::default()
        });
    }

    fn missing_token_for_pg(&self, snapshot: &ProcessSnapshot) -> bool {
        !self
            .incoming_flows
            .iter()
            .all(|sf| snapshot.tokens.contains_key(sf.id.as_str()))
    }

    fn try_execute_task<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        current_state: &State<'a>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
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
                            changes.push(StateChange {
                                consumes_tokens: slice::from_ref(inc_flow),
                                produces_tokens: &self.outgoing_flows,
                                consumes_message: Some(message),
                                produces_messages: &self.outgoing_message_flows,
                                ..Default::default()
                            });
                        }
                    } else {
                        // Handle normal task
                        changes.push(StateChange {
                            consumes_tokens: slice::from_ref(inc_flow),
                            produces_tokens: &self.outgoing_flows,
                            produces_messages: &self.outgoing_message_flows,
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    fn try_execute_intermediate_throw_event<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        current_state: &State<'a>,
        process: &'a Process,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        match &self.flow_node_type {
            FlowNodeType::IntermediateThrowEvent(Link(link_name)) => {
                self.try_execute_link_throw_event(snapshot, link_name, process, changes)
            }
            _ => self.try_execute_task(snapshot, current_state, changes),
        }
    }

    fn try_execute_link_throw_event<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        link_name: &str,
        process: &'a Process,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        let matching_link_catch_event = self.find_matching_link_catch_event(process, link_name);
        match matching_link_catch_event {
            None => {}
            Some(matching_link_catch_event) => {
                for inc_flow in self.incoming_flows.iter() {
                    match snapshot.tokens.get(inc_flow.id.as_str()) {
                        None => {}
                        Some(_) => {
                            changes.push(StateChange {
                                consumes_tokens: slice::from_ref(inc_flow),
                                produces_tokens: &matching_link_catch_event.outgoing_flows,
                                ..Default::default()
                            });
                        }
                    };
                }
            }
        }
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

    fn try_execute_exg<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(inc_flow.id.as_str()) {
                None => {}
                Some(_) => {
                    // Add one state with a token for each outgoing flow
                    for out_flow in self.outgoing_flows.iter() {
                        changes.push(StateChange {
                            consumes_tokens: slice::from_ref(inc_flow),
                            produces_tokens: slice::from_ref(out_flow),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    fn try_execute_end_event<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        event_type: &EventType,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(inc_flow.id.as_str()) {
                None => {}
                Some(_) => {
                    if event_type == &EventType::Terminate {
                        changes.push(StateChange {
                            // All tokens are removed due to terminate.
                            consumes_all_tokens: true,
                            records_end_event: Some(&self.id),
                            ..Default::default()
                        });
                        return;
                    }

                    changes.push(StateChange {
                        // Consume incoming token
                        consumes_tokens: slice::from_ref(inc_flow),
                        produces_messages: &self.outgoing_message_flows,
                        records_end_event: Some(&self.id),
                        ..Default::default()
                    });
                }
            }
        }
    }

    fn try_execute_intermediate_catch_event<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        current_state: &State<'a>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        match self.flow_node_type {
            FlowNodeType::IntermediateCatchEvent(EventType::Message) => {
                let message_flows_with_messages =
                    self.get_message_flows_with_message(current_state);
                if message_flows_with_messages.is_empty() {
                    return;
                }
                for inc_flow in self.incoming_flows.iter() {
                    match snapshot.tokens.get(inc_flow.id.as_str()) {
                        None => {}
                        Some(_) => {
                            for &message in message_flows_with_messages.iter() {
                                changes.push(StateChange {
                                    // Consume incoming token and message.
                                    consumes_tokens: slice::from_ref(inc_flow),
                                    consumes_message: Some(message),
                                    // Add outgoing tokens
                                    produces_tokens: &self.outgoing_flows,
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
            FlowNodeType::IntermediateCatchEvent(EventType::None) => {
                self.try_execute_task(snapshot, current_state, changes)
            }
            _ => {}
        }
    }

    fn get_message_flows_with_message(&self, current_state: &State) -> Vec<&str> {
        self.incoming_message_flows
            .iter()
            .filter_map(|inc_mf| {
                if current_state.messages.contains_key(inc_mf.id.as_str()) {
                    return Some(inc_mf.id.as_str());
                }
                None
            })
            .collect()
    }

    pub fn try_trigger_message_start_event<'a>(
        &'a self,
        process: &'a Process,
        current_state: &State<'a>,
    ) -> Vec<State<'a>> {
        let mut changes = vec![];
        self.collect_message_start_event_changes(current_state, &mut changes);
        changes
            .iter()
            .map(|change| change.apply(current_state, Instance::New(&process.id)))
            .collect()
    }

    /// Collects the changes of all possible triggers of this message start event, which each
    /// start a new process instance.
    pub(crate) fn collect_message_start_event_changes<'a>(
        &'a self,
        current_state: &State<'a>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        if current_state.messages.is_empty() {
            return;
        }
        for inc_mf in self.incoming_message_flows.iter() {
            let message_id = inc_mf.id.as_str();
            let message_count = current_state.messages.get(message_id);
            match message_count {
                None => {}
                Some(count) if *count > 0 => {
                    changes.push(StateChange {
                        consumes_message: Some(message_id),
                        // Add outgoing tokens to the snapshot of the new instance.
                        produces_tokens: &self.outgoing_flows,
                        ..Default::default()
                    });
                }
                Some(_) => {}
            }
        }
    }

    fn try_execute_evg<'a>(
        &'a self,
        snapshot: &ProcessSnapshot<'a>,
        current_state: &State<'a>,
        process: &'a Process,
        not_executed_activities: &mut HashSet<&str>,
        changes: &mut Vec<StateChange<'a>>,
    ) {
        // Currently only messages can trigger evgs.
        if current_state.messages.is_empty() {
            return;
        }
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
                            changes.push(StateChange {
                                // Consume incoming token and message
                                consumes_tokens: slice::from_ref(inc_flow),
                                consumes_message: Some(message),
                                // Add outgoing tokens
                                produces_tokens: &flow_node.outgoing_flows,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }

    /// Get the transition effect for this flow node.
    ///
    /// This describes what tokens/messages this node consumes and produces,
    /// which is used for independence checking in partial order reduction.
    ///
    /// # Arguments
    /// * `process_id` - The ID of the process this flow node belongs to
    /// * `snapshot` - The current process snapshot (to determine which incoming flow has a token)
    /// * `current_state` - The current state (to check message availability)
    /// * `process` - The process containing this flow node (needed to look up target events for event-based gateways)
    ///
    /// # Returns
    /// A `TransitionEffect` describing what this transition reads and writes
    pub fn get_transition_effect<'a>(
        &'a self,
        process_id: &'a str,
        snapshot: &ProcessSnapshot<'a>,
        current_state: &State<'a>,
        process: &'a Process,
    ) -> Option<TransitionEffect<'a>> {
        // Start events are not transitions (they create the initial state)
        if matches!(self.flow_node_type, FlowNodeType::StartEvent(_)) {
            return None;
        }

        let mut effect = TransitionEffect::new(&self.id, process_id);

        // Determine visibility based on flow node type
        effect.is_visible = match &self.flow_node_type {
            // Tasks are visible (for dead activity detection)
            FlowNodeType::Task(_) => true,
            // End events are visible (for proper completion)
            FlowNodeType::EndEvent(_) => true,
            // Gateways and intermediate events are generally invisible
            _ => false,
        };

        match &self.flow_node_type {
            FlowNodeType::StartEvent(_) => return None,

            FlowNodeType::Task(task_type) => {
                // Find which incoming flow has a token
                for inc_flow in &self.incoming_flows {
                    if snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        effect.consumes_tokens.insert(inc_flow.id.as_str());
                        break; // Tasks consume from one incoming flow
                    }
                }

                // Check if this is a receive task that needs a message
                if *task_type == TaskType::Receive {
                    for inc_mf in &self.incoming_message_flows {
                        if current_state.messages.contains_key(inc_mf.id.as_str()) {
                            effect.consumes_messages.insert(inc_mf.id.as_str());
                            break;
                        }
                    }
                    // If no message available, this transition is not enabled
                    if effect.consumes_messages.is_empty() {
                        return None;
                    }
                }

                // Add outgoing tokens
                for out_flow in &self.outgoing_flows {
                    effect.produces_tokens.insert(out_flow.id.as_str());
                }

                // Add outgoing messages
                for out_mf in &self.outgoing_message_flows {
                    effect.produces_messages.insert(out_mf.id.as_str());
                }
            }

            FlowNodeType::ParallelGateway => {
                // Parallel gateway requires ALL incoming tokens
                for inc_flow in &self.incoming_flows {
                    if !snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        return None; // Not enabled
                    }
                    effect.consumes_tokens.insert(inc_flow.id.as_str());
                }

                // Produces tokens on all outgoing flows
                for out_flow in &self.outgoing_flows {
                    effect.produces_tokens.insert(out_flow.id.as_str());
                }
            }

            FlowNodeType::ExclusiveGateway => {
                // Exclusive gateway consumes one incoming token, produces one outgoing
                for inc_flow in &self.incoming_flows {
                    if snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        effect.consumes_tokens.insert(inc_flow.id.as_str());
                        break;
                    }
                }

                // For independence analysis, we consider all possible outgoing tokens
                // as potentially produced (conservative over-approximation)
                for out_flow in &self.outgoing_flows {
                    effect.produces_tokens.insert(out_flow.id.as_str());
                }
            }

            FlowNodeType::EndEvent(event_type) => {
                // Consumes incoming token
                for inc_flow in &self.incoming_flows {
                    if snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        effect.consumes_tokens.insert(inc_flow.id.as_str());
                        break;
                    }
                }

                // Records end event execution
                effect.records_end_events.insert(&self.id);

                // Terminate end events are more complex but don't produce tokens
                if *event_type == EventType::Terminate {
                    // Terminate affects the entire process, making it dependent
                    // with all other transitions in the same process
                    // We handle this by marking it as visible
                    effect.is_visible = true;
                }

                // Add outgoing messages (message end events)
                for out_mf in &self.outgoing_message_flows {
                    effect.produces_messages.insert(out_mf.id.as_str());
                }
            }

            FlowNodeType::IntermediateThrowEvent(event_type) => {
                // Consumes incoming token
                for inc_flow in &self.incoming_flows {
                    if snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        effect.consumes_tokens.insert(inc_flow.id.as_str());
                        break;
                    }
                }

                match event_type {
                    EventType::Link(_) => {
                        // Link throw events produce tokens at the corresponding catch event
                        // This is process-local, so we mark the outgoing positions
                        // (The actual target is determined at execution time)
                        for out_flow in &self.outgoing_flows {
                            effect.produces_tokens.insert(out_flow.id.as_str());
                        }
                    }
                    EventType::Message => {
                        // Message throw events produce messages
                        for out_mf in &self.outgoing_message_flows {
                            effect.produces_messages.insert(out_mf.id.as_str());
                        }
                        for out_flow in &self.outgoing_flows {
                            effect.produces_tokens.insert(out_flow.id.as_str());
                        }
                    }
                    _ => {
                        for out_flow in &self.outgoing_flows {
                            effect.produces_tokens.insert(out_flow.id.as_str());
                        }
                    }
                }
            }

            FlowNodeType::IntermediateCatchEvent(event_type) => {
                // Consumes incoming token
                for inc_flow in &self.incoming_flows {
                    if snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        effect.consumes_tokens.insert(inc_flow.id.as_str());
                        break;
                    }
                }

                if *event_type == EventType::Message {
                    // Message catch events consume messages
                    for inc_mf in &self.incoming_message_flows {
                        if current_state.messages.contains_key(inc_mf.id.as_str()) {
                            effect.consumes_messages.insert(inc_mf.id.as_str());
                            break;
                        }
                    }
                    // If no message available, not enabled
                    if effect.consumes_messages.is_empty() {
                        return None;
                    }
                }

                for out_flow in &self.outgoing_flows {
                    effect.produces_tokens.insert(out_flow.id.as_str());
                }
            }

            FlowNodeType::EventBasedGateway => {
                // Event-based gateway is triggered by message events
                // It consumes incoming token and the message
                for inc_flow in &self.incoming_flows {
                    if snapshot.tokens.contains_key(inc_flow.id.as_str()) {
                        effect.consumes_tokens.insert(inc_flow.id.as_str());
                        break;
                    }
                }

                // Check for available messages by looking at the target events' incoming message flows
                // This matches the logic in try_execute_evg
                let mut has_matching_message = false;
                for out_flow in &self.outgoing_flows {
                    if let Some(target_flow_node) = process.flow_nodes.get(out_flow.target_idx) {
                        let message_flows_with_messages =
                            target_flow_node.get_message_flows_with_message(current_state);
                        if !message_flows_with_messages.is_empty() {
                            has_matching_message = true;
                            // Track which messages could be consumed
                            // (conservative: any message that matches a target event)
                            for message_id in message_flows_with_messages {
                                effect.consumes_messages.insert(message_id);
                            }
                        }
                    }
                }

                if !has_matching_message {
                    return None;
                }

                // Produces tokens based on which event triggers
                for out_flow in &self.outgoing_flows {
                    effect.produces_tokens.insert(out_flow.id.as_str());
                }
            }
        }

        // Only return effect if the transition is actually enabled
        // (has something to consume)
        // Note: StartEvents already return None at the beginning of this function,
        // so we don't need to check for them here.
        if effect.consumes_tokens.is_empty() && effect.consumes_messages.is_empty() {
            return None;
        }

        Some(effect)
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
