use rstest::rstest;
use rust_bpmn_analyzer::states::state_space::StateSpace;
use rust_bpmn_analyzer::{AmpleSetConfig, ModelCheckingResult, Property};
use std::collections::BTreeMap;

const PATH: &str = "tests/resources/integration/";

#[test]
fn test_stable_state_space1() {
    let file_path = PATH.to_string() + "p2.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, all_properties());
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.count_transitions());
    assert_eq!(4, result.property_results.len());
    assert_eq!(1, result.state_space.terminated_state_hashes.len());
    assert_eq!(0, get_unfulfilled_properties(result).len());
}

#[test]
fn test_stable_state_space2() {
    let file_path = PATH.to_string() + "p6_stuck.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, all_properties());
    assert_eq!(134, result.state_space.states.len());
    assert_eq!(454, result.state_space.count_transitions());
    assert_eq!(0, result.state_space.terminated_state_hashes.len());
    assert_eq!(
        vec![Property::OptionToComplete, Property::NoDeadActivities],
        get_unfulfilled_properties(result)
    );
}

#[test]
fn test_stable_state_space_with_messages() {
    let file_path = PATH.to_string() + "pools-message-flows.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, all_properties());
    assert_eq!(18, result.state_space.states.len());
    assert_eq!(21, result.state_space.count_transitions());
    assert_eq!(2, result.state_space.terminated_state_hashes.len());
    assert_eq!(0, get_unfulfilled_properties(result).len());
}

#[test]
fn test_stable_state_space_with_e020() {
    let file_path = PATH.to_string() + "e020.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, all_properties());
    assert_eq!(5356, result.state_space.states.len());
    assert_eq!(13556, result.state_space.count_transitions());
    assert_eq!(36, result.state_space.terminated_state_hashes.len());
    assert_eq!(0, get_unfulfilled_properties(result).len());
}

#[test]
fn test_persistent_messages() {
    let file_path = PATH.to_string() + "message_persistence.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, all_properties());
    assert_eq!(14, result.state_space.states.len());
    assert_eq!(17, result.state_space.count_transitions());
    // One terminated state is the first process completing before the second has started
    // The other is normal termination
    assert_eq!(2, result.state_space.terminated_state_hashes.len());
    assert_eq!(0, get_unfulfilled_properties(result).len());
}

// BPMN 2.0.2 Table 10.88 permits instance termination for unhandled errors.
// Check the analyzer's chosen policy on generated transitions, not just final states.
#[rstest]
#[case::full(false)]
#[case::por(true)]
fn test_error_end_event(#[case] por: bool) {
    let collaboration =
        rust_bpmn_analyzer::read_bpmn_from_file("tests/resources/unit/semantics/error_end.bpmn")
            .unwrap();
    let result = if por {
        rust_bpmn_analyzer::run_with_por(
            &collaboration,
            all_properties(),
            AmpleSetConfig::default(),
        )
        .result
    } else {
        rust_bpmn_analyzer::run(&collaboration, all_properties())
    };

    assert_eq!(result.state_space.states.len(), 14);
    assert_eq!(result.state_space.count_transitions(), 18);
    assert_eq!(result.state_space.terminated_state_hashes.len(), 4);

    let mut checked_error_transitions = 0;
    let mut cancelled_parallel_work = false;
    for (source_hash, transitions) in &result.state_space.transitions {
        let source = result.state_space.get_state(source_hash);
        for (node, target_hash) in transitions {
            if *node != "errorEnd" {
                continue;
            }
            checked_error_transitions += 1;
            let target = result.state_space.get_state(target_hash);
            let throwing_instance = source.snapshots.iter().find(|s| s.id == "process").unwrap();
            assert!(throwing_instance.tokens.contains_key("errorFlow"));
            cancelled_parallel_work |= throwing_instance.tokens.contains_key("taskFlow");
            assert!(
                target
                    .snapshots
                    .iter()
                    .find(|s| s.id == "process")
                    .unwrap()
                    .tokens
                    .is_empty()
            );
            assert_eq!(
                source.snapshots.iter().find(|s| s.id == "otherProcess"),
                target.snapshots.iter().find(|s| s.id == "otherProcess")
            );
            assert_eq!(target.messages, source.messages);
            let mut expected_counters = source.executed_end_event_counter.clone();
            expected_counters.insert("errorEnd", 1);
            assert_eq!(target.executed_end_event_counter, expected_counters);
            assert!(
                result
                    .state_space
                    .transitions
                    .get(target_hash)
                    .is_none_or(|successors| {
                        successors.iter().all(|(node, _)| *node == "otherEnd")
                    })
            );
        }
    }
    assert!(checked_error_transitions > 0);
    assert!(cancelled_parallel_work);
    for hash in &result.state_space.terminated_state_hashes {
        let state = result.state_space.get_state(hash);
        assert_eq!(state.executed_end_event_counter["errorEnd"], 1);
        assert_eq!(state.executed_end_event_counter["otherEnd"], 1);
    }
    assert_eq!(get_unfulfilled_properties(result), vec![]);
}

// BPMN 2.0.2 §13.5.1 creates a new instance on each message start;
// §13.5.6 keeps termination local to that instance.
#[rstest]
#[case::full(false)]
#[case::por(true)]
fn test_error_end_preserves_other_instances(#[case] por: bool) {
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(
        &(PATH.to_string() + "error_multiple_instances.bpmn"),
    )
    .unwrap();
    let result = if por {
        rust_bpmn_analyzer::run_with_por(&collaboration, vec![], AmpleSetConfig::default()).result
    } else {
        rust_bpmn_analyzer::run(&collaboration, vec![])
    };

    let mut checked_concurrent_instances = false;
    for (source_hash, transitions) in &result.state_space.transitions {
        let source = result.state_space.get_state(source_hash);
        let active_instances = source
            .snapshots
            .iter()
            .filter(|s| s.id == "worker" && !s.tokens.is_empty())
            .count();
        for (node, target_hash) in transitions {
            if *node != "errorEnd" {
                continue;
            }
            let target = result.state_space.get_state(target_hash);
            checked_concurrent_instances |= active_instances == 2;
            assert_eq!(target.snapshots.len(), source.snapshots.len());
            assert_eq!(
                target
                    .snapshots
                    .iter()
                    .filter(|s| s.id == "worker" && !s.tokens.is_empty())
                    .count(),
                active_instances - 1
            );
            assert_eq!(
                source.snapshots.iter().find(|s| s.id == "sender"),
                target.snapshots.iter().find(|s| s.id == "sender")
            );
            assert_eq!(target.messages, source.messages);
            assert_eq!(
                target.executed_end_event_counter["errorEnd"],
                source
                    .executed_end_event_counter
                    .get("errorEnd")
                    .copied()
                    .unwrap_or(0)
                    + 1
            );
        }
    }
    assert!(checked_concurrent_instances);
    assert!(
        result
            .state_space
            .states
            .values()
            .any(|state| { state.executed_end_event_counter.get("errorEnd") == Some(&2) })
    );
}

// BPMN 2.0.2 §10.5.3, p.245: end events consume tokens as they arrive, without synchronization.
#[rstest]
#[case::full(false)]
#[case::por(true)]
fn test_error_end_does_not_synchronize_incoming_flows(#[case] por: bool) {
    let collaboration =
        rust_bpmn_analyzer::read_bpmn_from_file(&(PATH.to_string() + "error_parallel_flows.bpmn"))
            .unwrap();
    let result = if por {
        rust_bpmn_analyzer::run_with_por(
            &collaboration,
            all_properties(),
            AmpleSetConfig::default(),
        )
        .result
    } else {
        rust_bpmn_analyzer::run(&collaboration, all_properties())
    };
    let space = &result.state_space;
    assert_eq!(space.states.len(), 4);
    assert_eq!(space.count_transitions(), 4);
    assert_eq!(space.terminated_state_hashes.len(), 1);

    assert_eq!(space.transitions[&space.start_state_hash].len(), 1);
    let split = successor(space, space.start_state_hash, "split");
    assert_eq!(
        space.get_state(&split).snapshots[0].tokens,
        BTreeMap::from([("directError", 1), ("deferredError", 1), ("blockedFlow", 1)])
    );
    let both_incoming = successor(space, split, "beforeError");
    assert_eq!(
        space.get_state(&both_incoming).snapshots[0].tokens,
        BTreeMap::from([("directError", 1), ("secondError", 1), ("blockedFlow", 1)])
    );
    let terminated = successor(space, split, "errorEnd");
    assert_eq!(successor(space, both_incoming, "errorEnd"), terminated);
    assert_eq!(space.transitions[&both_incoming].len(), 1);
    assert!(space.get_state(&terminated).is_terminated());
    assert_eq!(
        space.get_state(&terminated).executed_end_event_counter,
        BTreeMap::from([("errorEnd", 1)])
    );
    assert_eq!(
        get_unfulfilled_properties(result),
        vec![Property::NoDeadActivities]
    );
}

#[rstest]
#[case::full(false)]
#[case::por(true)]
fn test_error_end_preserves_sent_messages_but_cancels_pending_sends(#[case] por: bool) {
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(
        &(PATH.to_string() + "error_message_cancellation.bpmn"),
    )
    .unwrap();
    let result = if por {
        rust_bpmn_analyzer::run_with_por(
            &collaboration,
            all_properties(),
            AmpleSetConfig::default(),
        )
        .result
    } else {
        rust_bpmn_analyzer::run(&collaboration, all_properties())
    };
    let space = &result.state_space;
    let split = successor(space, space.start_state_hash, "split");
    let cancelled = successor(space, split, "errorEnd");
    let cancelled_state = space.get_state(&cancelled);
    assert!(cancelled_state.messages.is_empty());
    assert!(!cancelled_state.is_terminated());
    assert!(!space.transitions.contains_key(&cancelled));
    assert_eq!(
        cancelled_state
            .snapshots
            .iter()
            .find(|s| s.id == "receiver")
            .unwrap()
            .tokens,
        BTreeMap::from([("toReceive", 1)])
    );

    let sent = successor(space, split, "send");
    let error_after_send = successor(space, sent, "errorEnd");
    assert_eq!(
        space.get_state(&error_after_send).messages,
        BTreeMap::from([("message", 1)])
    );
    let received = successor(space, error_after_send, "receive");
    assert!(space.get_state(&received).messages.is_empty());
    let terminated = successor(space, received, "receiverEnd");
    assert!(space.get_state(&terminated).is_terminated());
    assert_eq!(
        space.get_state(&terminated).executed_end_event_counter,
        BTreeMap::from([("errorEnd", 1), ("receiverEnd", 1)])
    );
    assert_eq!(
        get_unfulfilled_properties(result),
        vec![Property::OptionToComplete]
    );
}

fn successor(space: &StateSpace, source: u64, node: &str) -> u64 {
    space.transitions[&source]
        .iter()
        .find(|(id, _)| *id == node)
        .unwrap()
        .1
}

#[test]
fn test_livelock() {
    let file_path = PATH.to_string() + "livelock.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, all_properties());
    assert_eq!(48195, result.state_space.states.len());
    assert_eq!(138391, result.state_space.count_transitions());
    assert_eq!(0, result.state_space.terminated_state_hashes.len());
    assert_eq!(
        vec![Property::Safeness, Property::OptionToComplete],
        get_unfulfilled_properties(result)
    );
}

fn all_properties() -> Vec<Property> {
    vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ]
}

fn get_unfulfilled_properties(result: ModelCheckingResult) -> Vec<Property> {
    result
        .property_results
        .into_iter()
        .filter_map(|property_result| {
            if !property_result.fulfilled {
                return Some(property_result.property);
            }
            None
        })
        .collect::<Vec<_>>()
}
