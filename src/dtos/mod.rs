use crate::states::state_space::{ProcessSnapshot, State, StateSpace};
use crate::{ModelCheckingResult, Property};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct CheckBPMNRequest {
    pub bpmn_file_content: String,
    pub properties_to_be_checked: Vec<Property>,
}

#[derive(Serialize)]
pub struct CheckBPMNResponse {
    pub property_results: Vec<MinimalPropertyResult>,
    pub unsupported_elements: Vec<String>,
}

impl CheckBPMNResponse {
    pub fn map_result(model_checking_result: ModelCheckingResult) -> CheckBPMNResponse {
        let property_results = model_checking_result
            .property_results
            .into_iter()
            .map(|mut result| {
                let state: &StateSpace = &model_checking_result.state_space;
                // Might not be needed once problematic elements and state hashes are put together in tuples.
                result.problematic_elements.sort();
                result.problematic_elements.dedup();
                MinimalPropertyResult {
                    fulfilled: result.fulfilled,
                    property: result.property,
                    problematic_elements: result.problematic_elements,
                    counter_example: CounterExample::new(result.problematic_state_hashes, state),
                }
            })
            .collect();

        CheckBPMNResponse {
            property_results,
            unsupported_elements: vec![],
        }
    }
}

#[derive(Serialize)]
pub struct MinimalPropertyResult {
    pub property: Property,
    pub fulfilled: bool,
    pub problematic_elements: Vec<String>,
    pub counter_example: Option<CounterExample>,
}

#[derive(Serialize)]
pub struct CounterExample {
    start_state: StateCopy,
    transitions: Vec<Transition>,
}

impl CounterExample {
    pub fn new(
        problematic_state_hashes: Vec<u64>,
        state_space: &StateSpace,
    ) -> Option<CounterExample> {
        match problematic_state_hashes.first() {
            None => None,
            Some(problematic_state) => match state_space.get_path_to_state(*problematic_state) {
                None => None,
                Some(path) => {
                    let transitions = path
                        .into_iter()
                        .map(|(label, state_hash)| Transition {
                            label: label.to_string(),
                            next_state: StateCopy::new(state_space.get_state(&state_hash)),
                        })
                        .collect();
                    let start_state = state_space.get_state(&state_space.start_state_hash);
                    Some(CounterExample {
                        start_state: StateCopy::new(start_state),
                        transitions,
                    })
                }
            },
        }
    }
}

#[derive(Serialize)]
struct Transition {
    label: String,
    // label is the executed flow node id
    next_state: StateCopy,
}

#[derive(Serialize)]
struct StateCopy {
    pub snapshots: Vec<ProcessSnapshotCopy>,
    pub messages: BTreeMap<String, u16>,
    pub executed_end_event_counter: BTreeMap<String, u16>,
}

impl StateCopy {
    fn new(state: &State) -> StateCopy {
        let snapshots = state
            .snapshots
            .iter()
            .map(|snapshot| ProcessSnapshotCopy::new(snapshot))
            .collect();

        let mut messages = BTreeMap::new();
        messages.extend(state.messages.iter().map(|(message, count)| {
            (message.to_string(), *count) // clone to make it owned
        }));

        let mut executed_end_event_counter = BTreeMap::new();
        executed_end_event_counter.extend(state.executed_end_event_counter.iter().map(
            |(end_event, count)| {
                (end_event.to_string(), *count) // clone to make it owned
            },
        ));

        StateCopy {
            snapshots,
            messages,
            executed_end_event_counter,
        }
    }
}

#[derive(Serialize)]
struct ProcessSnapshotCopy {
    pub id: String,
    pub tokens: BTreeMap<String, u16>,
}

impl ProcessSnapshotCopy {
    fn new(snapshot: &ProcessSnapshot) -> ProcessSnapshotCopy {
        let mut tokens = BTreeMap::new();
        tokens.extend(snapshot.tokens.iter().map(|(token, count)| {
            (token.to_string(), *count) // clone to make it owned
        }));
        ProcessSnapshotCopy {
            id: snapshot.id.to_string(),
            tokens,
        }
    }
}
