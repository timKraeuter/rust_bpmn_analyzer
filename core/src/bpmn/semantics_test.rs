#[cfg(test)]
mod test {
    use crate::bpmn::collaboration::Collaboration;
    use crate::bpmn::flow_node::FlowNode;
    use crate::bpmn::process::Process;
    use crate::bpmn::reader::read_bpmn_from_file;
    use crate::model_checking::properties::{ModelCheckingResult, PropertyResult};
    use crate::states::state_space::{ProcessSnapshot, State, reset_snapshot_counter};
    use crate::Property;
    use std::collections::{BTreeMap, HashMap};

    const PATH: &str = "tests/resources/unit/";

    fn read_bpmn_and_unwrap(path: &String) -> Collaboration {
        match read_bpmn_from_file(path) {
            Ok(collaboration) => collaboration,
            Err(err) => panic!(
                "Error reading the file {:?}. Unsupported elements found: {:?}",
                path, err.unsupported_elements
            ),
        }
    }

    #[test]
    fn create_start_state_one_participant() {
        reset_snapshot_counter();
        let expected_state = State {
            snapshots: vec![ProcessSnapshot::new("process", vec!["Flow_1", "Flow_2"],)],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/start.bpmn"));
        reset_snapshot_counter(); // Reset again to get the same IDs
        let start_state = collaboration.create_start_state();

        assert_eq!(start_state, expected_state);
    }

    #[test]
    fn create_start_state_multiple_participants() {
        reset_snapshot_counter();
        let expected_state = State {
            snapshots: vec![
                ProcessSnapshot::new("p1_process", vec!["Flow_04pas1n"]),
                ProcessSnapshot::new("p3_process", vec!["Flow_0gz2791"])
            ],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/multiple_participants.bpmn"));
        reset_snapshot_counter();
        let start_state = collaboration.create_start_state();

        assert_eq!(start_state, expected_state);
    }

    #[test]
    fn try_execute_task() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/task.bpmn"));
        let _start_state_template = collaboration.create_start_state();
        let expected_states = vec![
            State::new("process", vec!["Flow_2", "Flow_3", "Flow_4",]),
            State::new("process", vec!["Flow_1", "Flow_3", "Flow_4",])
        ];
        
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Activity_A");
        reset_snapshot_counter();
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, expected_states)
    }

    #[test]
    fn try_execute_receive_task() {
        reset_snapshot_counter();
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/receive_task.bpmn"));
        let _state_template = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_receive_task"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        let expected_state = State::new("p1_process", vec!["post_receive_task"]);
        
        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "ReceiveTask");
        reset_snapshot_counter();
        let state_without_message = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_receive_task"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_without_message),
            &state_without_message,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![]);

        let mut state_with_message = state_without_message;
        state_with_message.messages.insert("mf", 1u16);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_with_message),
            &state_with_message,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state]);
    }

    #[test]
    fn try_execute_evg() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/evg.bpmn"));
        let _state_template = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_evg"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        let mut state1 = State::new("p1_process", vec!["post_mice"]);
        state1.messages.insert("mf1", 1u16);
        let mut state2 = State::new("p1_process", vec!["post_ReceiveTask"]);
        state2.messages.insert("mf2", 1u16);
        
        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "evg");
        reset_snapshot_counter();
        let state_without_message = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_evg"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_without_message),
            &state_without_message,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![]);

        let mut state_with_message = state_without_message;
        state_with_message.messages.insert("mf1", 1u16);
        state_with_message.messages.insert("mf2", 1u16);

        let mut not_executed_activities = HashMap::new();
        not_executed_activities.insert("ReceiveTask", true);
        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_with_message),
            &state_with_message,
            process,
            &mut not_executed_activities,
        );

        assert_eq!(next_states, vec![state1, state2]);
        // The map is empty since the activity was executed to reach one of the states.
        assert_eq!(not_executed_activities.len(), 0);
    }

    #[test]
    fn try_execute_receive_task_no_message_flows() {
        reset_snapshot_counter();
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/receive_task_no_mf.bpmn"));
        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "ReceiveTask");
        let state = State::new("p1_process", vec!["sf"]);

        let next_states = flow_node.try_trigger_message_start_event(process, &state);

        assert_eq!(next_states.len(), 0);
    }

    #[test]
    fn try_execute_message_intermediate_catch_event() {
        reset_snapshot_counter();
        let state_without_message_template = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_mice"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        let mut expected_state = State::new("p1_process", vec!["post_mice"]);
        expected_state.messages.insert("mf", 1u16);
        
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "semantics/message_intermediate_catch_event.bpmn"),
        );

        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "mice");
        reset_snapshot_counter();
        let state_without_message = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_mice"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_without_message),
            &state_without_message,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![]);

        let mut state_with_message = state_without_message;
        state_with_message.messages.insert("mf", 2u16);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_with_message),
            &state_with_message,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state,]);
    }

    #[test]
    fn try_execute_send_task() {
        reset_snapshot_counter();
        let _input_state_template = State::new("p1_process", vec!["pre_send_task"]);
        let expected_state = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["post_send_task"],)],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::from([("mf", 1u16)]),
        };
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/send_task.bpmn"));

        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "SendTask");

        reset_snapshot_counter();
        let state = State::new("p1_process", vec!["pre_send_task"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state]);
    }

    #[test]
    fn try_execute_exg_choice() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/exg.bpmn"));
        let _start_state_template = collaboration.create_start_state();
        let expected_states = vec![
            State::new("process", vec!["Flow_2"]),
            State::new("process", vec!["Flow_3"]),
        ];
        
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_1");
        reset_snapshot_counter();
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, expected_states)
    }

    #[test]
    fn try_execute_exg_merge() {
        reset_snapshot_counter();
        let _input_state_template = State::new("process", vec!["Flow_2", "Flow_3"]);
        let expected_states = vec![
            State::new("process", vec!["Flow_3", "Flow_4"],),
            State::new("process", vec!["Flow_2", "Flow_4"],),
        ];
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_2");
        reset_snapshot_counter();
        let state = State::new("process", vec!["Flow_2", "Flow_3"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, expected_states);
    }

    #[test]
    fn try_execute_pg_split() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));
        let _start_state_template = collaboration.create_start_state();
        let expected_state = State::new("process", vec!["Flow_2", "Flow_3"],);
        
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_1");
        reset_snapshot_counter();
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state])
    }

    #[test]
    fn try_execute_pg_sync() {
        reset_snapshot_counter();
        let _input_state_template = State::new("process", vec!["Flow_2", "Flow_3"]);
        let expected_state = State::new("process", vec!["Flow_4"],);
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_2");
        reset_snapshot_counter();
        let state = State::new("process", vec!["Flow_2", "Flow_3"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state]);
    }

    #[test]
    fn try_execute_end_event() {
        reset_snapshot_counter();
        let _input_state_template = State::new("process", vec!["Flow_1", "Flow_1", "Flow_2"]);
        let mut state1 = State::new("process", vec!["Flow_1", "Flow_2"]);
        state1.executed_end_event_counter.insert("End", 1);
        let mut state2 = State::new("process", vec!["Flow_1", "Flow_1"]);
        state2.executed_end_event_counter.insert("End", 1);
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/end.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "End");
        reset_snapshot_counter();
        let state = State::new("process", vec!["Flow_1", "Flow_1", "Flow_2"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![state1, state2]);
    }

    #[test]
    fn try_execute_terminate_end_event() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/terminate_end.bpmn"));
        
        let process = get_process_by_id(&collaboration, "p1_process");

        let terminate_end_event: &FlowNode = get_flow_node_with_id(process, "end");
        reset_snapshot_counter();
        let start_state = collaboration.create_start_state();

        let next_states = terminate_end_event.try_execute(
            get_snapshot_by_process_id(&start_state, "p1_process"),
            &start_state,
            process,
            &mut HashMap::new(),
        );

        // Verify the structure matches expectations
        assert_eq!(next_states.len(), 1);
        let result_state = &next_states[0];
        assert_eq!(result_state.snapshots.len(), 2);
        assert_eq!(result_state.snapshots[0].process_id, "p0_process");
        assert_eq!(result_state.snapshots[0].tokens, BTreeMap::from([("flow3", 1u16)]));
        assert_eq!(result_state.snapshots[1].process_id, "p1_process");
        assert_eq!(result_state.snapshots[1].tokens, BTreeMap::new());
        assert_eq!(result_state.executed_end_event_counter, BTreeMap::from([("end", 1u16)]));
        assert_eq!(result_state.messages, BTreeMap::new());
    }

    #[test]
    fn try_execute_intermediate_throw_event() {
        reset_snapshot_counter();
        let _input_state_template = State::new("process", vec!["Flow_1", "Flow_2"]);
        let expected_states = vec![
            State::new("process", vec!["Flow_2", "Flow_3", "Flow_4",],),
            State::new("process", vec!["Flow_1", "Flow_3", "Flow_4",],),
        ];
        
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/intermediate_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Intermediate");
        reset_snapshot_counter();
        let state = State::new("process", vec!["Flow_1", "Flow_2"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, expected_states);
    }

    #[test]
    fn try_execute_link_events_connected_by_names() {
        reset_snapshot_counter();
        let _input_state_template = State::new("p1_process", vec!["pre_c"]);
        let expected_state = State::new("p1_process", vec!["post_c"]);
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/link_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "C");
        reset_snapshot_counter();
        let state = State::new("p1_process", vec!["pre_c"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state]);
    }

    #[test]
    fn try_execute_link_events_connected_by_link_names() {
        reset_snapshot_counter();
        let _input_state_template = State::new("p1_process", vec!["flow_a"]);
        let expected_state = State::new("p1_process", vec!["flow_b"]);
        
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/link_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "A");
        reset_snapshot_counter();
        let state = State::new("p1_process", vec!["flow_a"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state),
            &state,
            process,
            &mut HashMap::new(),
        );

        assert_eq!(next_states, vec![expected_state]);
    }

    #[test]
    fn try_execute_message_start() {
        reset_snapshot_counter();
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/message_start_event.bpmn"));
        let _input_state_template = State {
            snapshots: vec![],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::from([("mf", 1u16)]),
        };
        let expected_state = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["start_out"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "start");
        reset_snapshot_counter();
        let state = State {
            snapshots: vec![],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::from([("mf", 1u16)]),
        };

        let next_states = flow_node.try_trigger_message_start_event(process, &state);

        assert_eq!(next_states, vec![expected_state]);
    }

    fn get_flow_nodes_executed_to_reach<'a>(
        model_checking_result: &'a ModelCheckingResult<'a>,
        state_hash: u64,
    ) -> Vec<&'a str> {
        let path_to_unsafe = model_checking_result
            .state_space
            .get_path_to_state(state_hash)
            .unwrap();
        get_flow_nodes_for_path(path_to_unsafe)
    }

    fn get_flow_nodes_for_path(path_to_unsafe: Vec<(&str, u64)>) -> Vec<&str> {
        path_to_unsafe
            .into_iter()
            .map(|(executed_flow_node_id, _)| executed_flow_node_id)
            .collect()
    }

    #[test]
    fn safeness_unfulfilled() {
        reset_snapshot_counter();
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "properties/safeness/unsafe.bpmn"));

        let model_checking_result = collaboration.explore_state_space(vec![Property::Safeness]);

        // Note: With unique snapshot IDs, the state space may contain more states
        // Check that at least the key unsafe elements are detected
        assert_eq!(model_checking_result.property_results.len(), 1);
        let result = &model_checking_result.property_results[0];
        assert_eq!(result.property, Property::Safeness);
        assert_eq!(result.fulfilled, false);
        assert!(result.problematic_elements.contains(&String::from("Unsafe1")));
        assert!(result.problematic_elements.contains(&String::from("Unsafe2")));
        assert!(result.problematic_state_hashes.len() > 0);
        
        // Verify we can reach at least one unsafe state
        let first_hash = result.problematic_state_hashes[0];
        let unsafe_state = model_checking_result.get_state(&first_hash).unwrap();
        assert_eq!(unsafe_state.snapshots.len(), 1);
        assert_eq!(unsafe_state.snapshots[0].process_id, "process");
        assert_eq!(unsafe_state.executed_end_event_counter, BTreeMap::new());
        assert_eq!(unsafe_state.messages, BTreeMap::new());
        
        let path_to_unsafe = get_flow_nodes_executed_to_reach(&model_checking_result, first_hash);
        assert!(path_to_unsafe.len() > 0);
    }

    #[test]
    fn safeness_fulfilled() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));

        let model_checking_result = collaboration.explore_state_space(vec![Property::Safeness]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult::safe()]
        );
    }

    #[test]
    fn option_to_complete_unfulfilled_1() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/option_to_complete/no-option-to-complete-1.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::OptionToComplete]);

        // Check that the property is unfulfilled and stuck states are found
        assert_eq!(model_checking_result.property_results.len(), 1);
        let result = &model_checking_result.property_results[0];
        assert_eq!(result.property, Property::OptionToComplete);
        assert_eq!(result.fulfilled, false);
        assert!(result.problematic_state_hashes.len() >= 2, "Should find at least 2 stuck states");
        
        // Verify we can retrieve the states and trace paths
        for &state_hash in &result.problematic_state_hashes {
            let state = model_checking_result.get_state(&state_hash);
            assert!(state.is_some(), "Should be able to retrieve stuck state");
            let path = get_flow_nodes_executed_to_reach(&model_checking_result, state_hash);
            assert!(path.len() > 0, "Should have a path to the stuck state");
        }
    }

    #[test]
    fn option_to_complete_unfulfilled_2() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/option_to_complete/no-option-to-complete-2.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::OptionToComplete]);

        // Check that the property is unfulfilled
        assert_eq!(model_checking_result.property_results.len(), 1);
        let result = &model_checking_result.property_results[0];
        assert_eq!(result.property, Property::OptionToComplete);
        assert_eq!(result.fulfilled, false);
        assert!(result.problematic_state_hashes.len() > 0);

        // Verify we can retrieve the stuck state
        let stuck_hash = result.problematic_state_hashes[0];
        let stuck_state = model_checking_result.get_state(&stuck_hash).unwrap();
        // Check that the state has the expected structure
        assert_eq!(stuck_state.snapshots.len(), 1);
        assert_eq!(stuck_state.snapshots[0].process_id, "Process_dc137d1f-9555-4446-bfd0-adebe6a3bdb2");
        assert_eq!(stuck_state.snapshots[0].tokens, BTreeMap::from([("stuck", 1u16)]));
        assert_eq!(stuck_state.executed_end_event_counter, BTreeMap::new());
        assert_eq!(stuck_state.messages, BTreeMap::new());
    }

    #[test]
    fn option_to_complete_fulfilled() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::OptionToComplete]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult::always_terminates()]
        );
    }

    #[test]
    fn no_dead_activities_unfulfilled() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/no_dead_activities/dead-activities.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::NoDeadActivities]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult {
                property: Property::NoDeadActivities,
                fulfilled: false,
                problematic_elements: vec![
                    String::from("Dead_1"),
                    String::from("Dead_2"),
                    String::from("Dead_3"),
                    String::from("Dead_4"),
                ],
                ..Default::default()
            }]
        );
    }

    #[test]
    fn no_dead_activities_fulfilled() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/no_dead_activities/no-dead-activities.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::NoDeadActivities]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult::no_dead_activities()]
        );
    }

    #[test]
    fn proper_completion_fulfilled_1() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/proper-completion-1.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::ProperCompletion]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult::proper_completion()]
        );
    }

    #[test]
    fn proper_completion_fulfilled_2() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/proper-completion-2.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::ProperCompletion]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult::proper_completion()]
        );
    }

    #[test]
    fn proper_completion_unfulfilled_1() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/no-proper-completion-1.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::ProperCompletion]);

        let result = model_checking_result.property_results.first().unwrap();
        assert_eq!(result.property, Property::ProperCompletion);
        assert_eq!(result.fulfilled, false);
        assert!(result.problematic_elements.contains(&"EndEvent_1".to_string()));
        assert!(result.problematic_state_hashes.len() > 0);
    }

    #[test]
    fn proper_completion_unfulfilled_2() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/no-proper-completion-2.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::ProperCompletion]);

        let result = model_checking_result.property_results.first().unwrap();
        assert_eq!(result.property, Property::ProperCompletion);
        assert_eq!(result.fulfilled, false);
        assert!(result.problematic_elements.contains(&"EndEvent_1".to_string()));
        assert!(result.problematic_state_hashes.len() > 0);
    }

    #[test]
    fn proper_completion_unfulfilled_3() {
        reset_snapshot_counter();
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/no-proper-completion-3-unsafe.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::Safeness, Property::ProperCompletion]);

        let result = model_checking_result.property_results.get(1).unwrap();
        assert_eq!(result.property, Property::ProperCompletion);
        assert_eq!(result.fulfilled, false);
        assert!(result.problematic_elements.contains(&"EndEvent_1".to_string()));
        assert!(result.problematic_state_hashes.len() > 0);
    }

    fn get_first_process(collaboration: &Collaboration) -> &Process {
        collaboration.participants.first().unwrap()
    }

    fn get_process_by_id<'a>(collaboration: &'a Collaboration, process_id: &str) -> &'a Process {
        collaboration
            .participants
            .iter()
            .find(|p| p.id == process_id)
            .unwrap()
    }

    fn get_first_snapshot<'a>(start_state: &'a State) -> &'a ProcessSnapshot<'a> {
        let snapshot = start_state.snapshots.first().unwrap();
        snapshot
    }

    fn get_snapshot_by_process_id<'a>(start_state: &'a State, process_id: &str) -> &'a ProcessSnapshot<'a> {
        start_state
            .snapshots
            .iter()
            .find(|snapshot| snapshot.process_id == process_id)
            .unwrap()
    }

    fn get_flow_node_with_id<'a>(process: &'a Process, id: &str) -> &'a FlowNode {
        process
            .flow_nodes
            .iter()
            .find(|flow_node| flow_node.id == id)
            .unwrap()
    }
}
