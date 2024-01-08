use crate::states::state_space::{ProcessSnapshot, State};

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
    pub fn new(id: String, flow_node_type: FlowNodeType) -> FlowNode {
        FlowNode {
            id,
            flow_node_type,
            incoming_flows: vec![],
            outgoing_flows: vec![],
        }
    }
    pub fn add_outgoing_flow(&mut self, sf: SequenceFlow) {
        self.outgoing_flows.push(sf);
    }
    pub fn add_incoming_flow(&mut self, sf: SequenceFlow) {
        self.incoming_flows.push(sf);
    }
    pub fn try_execute(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        match self.flow_node_type {
            FlowNodeType::StartEvent => {
                vec![]
            }
            FlowNodeType::Task => self.try_execute_task(snapshot, current_state),
            FlowNodeType::IntermediateThrowEvent => {
                self.try_intermediate_throw_event(snapshot, current_state)
            }
            FlowNodeType::ExclusiveGateway => self.try_execute_exg(snapshot, current_state),
            FlowNodeType::ParallelGateway => self.try_execute_pg(snapshot, current_state),
            FlowNodeType::EndEvent => self.try_execute_end_event(snapshot, current_state),
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
            new_snapshot.delete_token(in_sf.id.clone());
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
        // Clone should be avoided.
        let mut snapshots = current_state.snapshots.clone();
        // Remove the snapshot
        let index = snapshots
            .iter()
            .position(|sp| snapshot.id == sp.id)
            .expect("Snapshot not found!");
        snapshots.swap_remove(index);

        State {
            snapshots,
            executed_end_event_counter: current_state.executed_end_event_counter.clone(),
        }
    }

    fn add_outgoing_tokens(&self, snapshot: &mut ProcessSnapshot) {
        for out_flow in self.outgoing_flows.iter() {
            snapshot.add_token(out_flow.id.clone());
        }
    }
    fn try_execute_task(&self, snapshot: &ProcessSnapshot, current_state: &State) -> Vec<State> {
        let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Add new state
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
    fn try_intermediate_throw_event(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
    ) -> Vec<State> {
        // Currently the same as task but event types will change this.
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
                        new_snapshot.add_token(out_flow.id.clone());
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
        snapshot.delete_token(token.to_string());
        snapshot
    }
    fn try_execute_end_event(
        &self,
        snapshot: &ProcessSnapshot,
        current_state: &State,
    ) -> Vec<State> {
        let mut new_states: Vec<State> = Vec::with_capacity(1); // Usually there is only one incoming flow, i.e., max 1 new state.
        for inc_flow in self.incoming_flows.iter() {
            match snapshot.tokens.get(&inc_flow.id) {
                None => {}
                Some(_) => {
                    // Consume incoming token
                    let mut new_state =
                        Self::create_new_state_without_snapshot(snapshot, current_state);
                    let new_snapshot =
                        Self::create_new_snapshot_without_token(snapshot, &inc_flow.id);
                    // Add outgoing token
                    new_state.snapshots.push(new_snapshot);
                    self.record_end_event_execution(&mut new_state);

                    new_states.push(new_state);
                }
            }
        }
        new_states
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
