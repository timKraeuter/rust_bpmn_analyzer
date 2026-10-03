use crate::bpmn::flow_node::FlowNodeType::StartEvent;
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType, MessageFlow, TaskType};
use crate::bpmn::process::Process;
use crate::model_checking::properties::{
    ModelCheckingResult, Property, PropertyResult, check_on_the_fly_properties,
    determine_properties,
};
use crate::states::state_space::{ProcessSnapshot, State, StateSpace};
use crate::states::successor::{Instance, StateChange, SuccessorBuilder};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, HashSet, VecDeque};

#[derive(Debug, PartialEq)]
pub struct Collaboration {
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

    pub fn explore_state_space(&self, properties: Vec<Property>) -> ModelCheckingResult<'_> {
        let mut property_results = vec![];
        let mut not_executed_activities = self.get_all_tasks();

        let mut seen_state_hashes = FxHashSet::default();
        let start_state = self.create_start_state();
        let start_state_hash = start_state.calc_hash();
        seen_state_hashes.insert(start_state_hash);

        let mut state_space = StateSpace {
            start_state_hash,
            terminated_state_hashes: vec![],
            states: FxHashMap::default(),
            transitions: FxHashMap::default(),
        };

        let mut unexplored_states = VecDeque::new();
        unexplored_states.push_back((start_state_hash, start_state));
        let mut buffers = ExplorationBuffers::default();

        while !unexplored_states.is_empty() {
            match unexplored_states.pop_front() {
                None => {}
                Some((current_state_hash, current_state)) => {
                    // Explore the state
                    self.explore_state(&current_state, &mut not_executed_activities, &mut buffers);

                    let mut transitions = Vec::with_capacity(buffers.executions.len());
                    for (flow_node_id, executor, change) in buffers.executions.drain(..) {
                        let instance = match executor {
                            Executor::Snapshot(index) => {
                                Instance::Existing(&current_state.snapshots[index])
                            }
                            Executor::NewInstance(process_id) => Instance::New(process_id),
                        };
                        let successor =
                            buffers
                                .successor_builder
                                .apply(&current_state, instance, &change);
                        let new_hash = successor.calc_hash();
                        // Check if we know the state already
                        if seen_state_hashes.insert(new_hash) {
                            // State is new. Only new states are built to not clone known states.
                            let new_state = successor.build();
                            debug_assert_eq!(new_hash, new_state.calc_hash());
                            unexplored_states.push_back((new_hash, new_state));
                        }
                        // Remember states to make transitions.
                        transitions.push((flow_node_id, new_hash));
                    }
                    // Do stuff for model checking
                    let property_result = check_on_the_fly_properties(
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
                    match property_result {
                        Ok(_) => {}
                        Err(e) => {
                            property_results.push(PropertyResult {
                                property: Property::OptionToComplete,
                                problematic_state_hashes: vec![current_state_hash],
                                problematic_elements: vec![e.overflowing_position],
                                fulfilled: false,
                            });
                            break;
                        }
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

    pub fn get_all_tasks(&self) -> HashSet<&str> {
        let mut flow_nodes = HashSet::new();
        self.participants.iter().for_each(|process| {
            process
                .flow_nodes
                .iter()
                .filter(|flow_node| {
                    flow_node.flow_node_type == FlowNodeType::Task(TaskType::Default)
                        || flow_node.flow_node_type == FlowNodeType::Task(TaskType::Receive)
                })
                .for_each(|flow_node| {
                    flow_nodes.insert(flow_node.id.as_str());
                })
        });
        flow_nodes
    }

    pub fn create_start_state(&self) -> State<'_> {
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
        not_executed_activities: &mut HashSet<&str>,
        buffers: &mut ExplorationBuffers<'a>,
    ) {
        if !state.messages.is_empty() {
            self.try_trigger_message_start_events(state, buffers);
        }

        for (snapshot_index, snapshot) in state.snapshots.iter().enumerate() {
            // Find participant for snapshot, could also be hashmap but usually not a long list.
            let process = self
                .participants
                .iter()
                .find(|process| process.id == snapshot.id);
            match process {
                None => {
                    panic!("No process found for snapshot with id \"{}\"", snapshot.id)
                }
                Some(process) => {
                    Collaboration::collect_flow_node_indexes_with_incoming_tokens(
                        snapshot,
                        process,
                        &mut buffers.flow_node_indexes,
                    );
                    for flow_node in buffers
                        .flow_node_indexes
                        .iter()
                        .filter_map(|&flow_node_idx| process.flow_nodes.get(*flow_node_idx))
                    {
                        flow_node.collect_state_changes(
                            snapshot,
                            state,
                            process,
                            not_executed_activities,
                            &mut buffers.changes,
                        );

                        Self::record_executed_activities(
                            not_executed_activities,
                            flow_node,
                            !buffers.changes.is_empty(),
                        );

                        // Would want to check if the state has been explored here not later to not take up unnecessary memory. But we still want to add the transitions.
                        buffers
                            .executions
                            .extend(buffers.changes.drain(..).map(|change| {
                                (
                                    flow_node.id.as_str(),
                                    Executor::Snapshot(snapshot_index),
                                    change,
                                )
                            }));
                    }
                }
            }
        }
    }

    pub(crate) fn get_flow_node_indexes_with_incoming_tokens<'a>(
        snapshot: &ProcessSnapshot,
        process: &'a Process,
    ) -> Vec<&'a usize> {
        let mut flow_node_indexes = vec![];
        Collaboration::collect_flow_node_indexes_with_incoming_tokens(
            snapshot,
            process,
            &mut flow_node_indexes,
        );
        flow_node_indexes
    }

    fn collect_flow_node_indexes_with_incoming_tokens<'a>(
        snapshot: &ProcessSnapshot,
        process: &'a Process,
        flow_node_indexes: &mut Vec<&'a usize>,
    ) {
        flow_node_indexes.clear();
        flow_node_indexes.extend(
            snapshot
                .tokens
                .iter()
                .filter_map(|(&token_position, _)| process.sequence_flow_index.get(token_position)),
        );
        flow_node_indexes.sort();
        flow_node_indexes.dedup(); // Do not try to execute a flow node twice.
    }

    fn try_trigger_message_start_events<'a>(
        &'a self,
        state: &State<'a>,
        buffers: &mut ExplorationBuffers<'a>,
    ) {
        self.participants.iter().for_each(|process| {
            process
                .flow_nodes
                .iter()
                .filter(|flow_node| flow_node.flow_node_type == StartEvent(EventType::Message))
                .for_each(|message_start_event| {
                    message_start_event
                        .collect_message_start_event_changes(state, &mut buffers.changes);
                    // Would want to check if the state has been explored here not later to not take up unnecessary memory. But we still want to add the transitions.
                    buffers
                        .executions
                        .extend(buffers.changes.drain(..).map(|change| {
                            (
                                message_start_event.id.as_str(),
                                Executor::NewInstance(process.id.as_str()),
                                change,
                            )
                        }));
                })
        });
    }

    pub(crate) fn record_executed_activities(
        not_executed_activities: &mut HashSet<&str>,
        flow_node: &FlowNode,
        executed: bool,
    ) {
        // Removing hashes the id, which is unnecessary once all activities were executed.
        if executed
            && !not_executed_activities.is_empty()
            && (flow_node.flow_node_type == FlowNodeType::Task(TaskType::Default)
                || flow_node.flow_node_type == FlowNodeType::Task(TaskType::Receive))
        {
            not_executed_activities.remove(flow_node.id.as_str());
        }
    }
}

/// The process instance executing a flow node in the explored state, see [`Instance`].
#[derive(Debug)]
enum Executor<'a> {
    /// Index of the executing snapshot.
    Snapshot(usize),
    /// Process id of the instance started by a message start event.
    NewInstance(&'a str),
}

/// Buffers reused when exploring states.
#[derive(Debug, Default)]
struct ExplorationBuffers<'a> {
    flow_node_indexes: Vec<&'a usize>,
    changes: Vec<StateChange<'a>>,
    /// Executions of flow nodes in the explored state (executed flow node id, executor, change).
    executions: Vec<(&'a str, Executor<'a>, StateChange<'a>)>,
    successor_builder: SuccessorBuilder<'a>,
}
