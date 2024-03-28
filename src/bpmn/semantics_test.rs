#[cfg(test)]
mod test {
    use crate::bpmn::collaboration::Collaboration;
    use crate::bpmn::flow_node::FlowNode;
    use crate::bpmn::process::Process;
    use crate::bpmn::reader::read_bpmn_file;
    use crate::model_checking::properties::{ModelCheckingResult, PropertyResult};
    use crate::states::state_space::{ProcessSnapshot, State};
    use crate::Property;
    use std::collections::BTreeMap;

    const PATH: &str = "tests/resources/unit/";

    fn read_bpmn_and_unwrap(path: &String) -> Collaboration {
        match read_bpmn_file(path) {
            Ok(collaboration) => collaboration,
            Err(err) => panic!(
                "Error reading the file {:?}. Unsupported elements found: {:?}",
                path, err.unsupported_elements
            ),
        }
    }

    #[test]
    fn create_start_state_one_participant() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/start.bpmn"));
        let start_state = collaboration.create_start_state();

        assert_eq!(
            start_state,
            State {
                snapshots: vec![ProcessSnapshot::new("process", vec!["Flow_1", "Flow_2"],)],
                executed_end_event_counter: BTreeMap::new(),
                messages: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn create_start_state_multiple_participants() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/multiple_participants.bpmn"));
        let start_state = collaboration.create_start_state();

        assert_eq!(
            start_state,
            State {
                snapshots: vec![
                    ProcessSnapshot::new("p1_process", vec!["Flow_04pas1n"]),
                    ProcessSnapshot::new("p3_process", vec!["Flow_0gz2791"])
                ],
                executed_end_event_counter: BTreeMap::new(),
                messages: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn try_execute_task() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/task.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Activity_A");
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![
                State::new("process", vec!["Flow_2", "Flow_3", "Flow_4",]),
                State::new("process", vec!["Flow_1", "Flow_3", "Flow_4",])
            ]
        )
    }

    #[test]
    fn try_execute_receive_task() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/receive_task.bpmn"));

        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "ReceiveTask");
        let state_without_message = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_receive_task"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_without_message),
            &state_without_message,
            &collaboration,
        );

        assert_eq!(next_states, vec![]);

        let mut state_with_message = state_without_message;
        state_with_message.messages.insert(String::from("mf"), 1u16);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_with_message),
            &state_with_message,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![State::new("p1_process", vec!["post_receive_task"]),]
        );
    }

    #[test]
    fn try_execute_evg() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/evg.bpmn"));

        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "evg");
        let state_without_message = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_evg"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_without_message),
            &state_without_message,
            &collaboration,
        );

        assert_eq!(next_states, vec![]);

        let mut state_with_message = state_without_message;
        state_with_message
            .messages
            .insert(String::from("mf1"), 1u16);
        state_with_message
            .messages
            .insert(String::from("mf2"), 1u16);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_with_message),
            &state_with_message,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![
                State::new("p1_process", vec!["post_mice"]),
                State::new("p1_process", vec!["post_ReceiveTask"])
            ]
        );
    }

    #[test]
    fn try_execute_receive_task_no_message_flows() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/receive_task_no_mf.bpmn"));
        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "ReceiveTask");
        let start_state = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["sf"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_trigger_message_start_event(process, &start_state);

        assert_eq!(next_states.len(), 0);
    }

    #[test]
    fn try_execute_message_intermediate_catch_event() {
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "semantics/message_intermediate_catch_event.bpmn"),
        );

        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "mice");
        let state_without_message = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_mice"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_without_message),
            &state_without_message,
            &collaboration,
        );

        assert_eq!(next_states, vec![]);

        let mut state_with_message = state_without_message;
        state_with_message.messages.insert(String::from("mf"), 1u16);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&state_with_message),
            &state_with_message,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![State::new("p1_process", vec!["post_mice"]),]
        );
    }

    #[test]
    fn try_execute_send_task() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/send_task.bpmn"));

        let process = get_process_by_id(&collaboration, "p1_process");

        let flow_node: &FlowNode = get_flow_node_with_id(process, "SendTask");
        let state = State {
            snapshots: vec![ProcessSnapshot::new("p1_process", vec!["pre_send_task"])],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let next_states = flow_node.try_execute(get_first_snapshot(&state), &state, &collaboration);

        assert_eq!(
            next_states,
            vec![State {
                snapshots: vec![ProcessSnapshot::new("p1_process", vec!["post_send_task"],)],
                executed_end_event_counter: BTreeMap::new(),
                messages: BTreeMap::from([(String::from("mf"), 1u16)]),
            }]
        );
    }

    #[test]
    fn try_execute_exg_choice() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_1");
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![
                State::new("process", vec!["Flow_2"]),
                State::new("process", vec!["Flow_3"]),
            ]
        )
    }

    #[test]
    fn try_execute_exg_merge() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_2");
        let start_state = State::new("process", vec!["Flow_2", "Flow_3"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![
                State::new("process", vec!["Flow_3", "Flow_4"],),
                State::new("process", vec!["Flow_2", "Flow_4"],),
            ]
        );
    }

    #[test]
    fn try_execute_pg_split() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_1");
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![State::new("process", vec!["Flow_2", "Flow_3"],),]
        )
    }

    #[test]
    fn try_execute_pg_sync() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Gateway_2");
        let start_state = State::new("process", vec!["Flow_2", "Flow_3"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        assert_eq!(next_states, vec![State::new("process", vec!["Flow_4"],),]);
    }

    #[test]
    fn try_execute_end_event() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/end.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "End");
        let start_state = State::new("process", vec!["Flow_1", "Flow_1", "Flow_2"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        let mut state1 = State::new("process", vec!["Flow_1", "Flow_2"]);
        state1
            .executed_end_event_counter
            .insert("End".to_string(), 1);
        let mut state2 = State::new("process", vec!["Flow_1", "Flow_1"]);
        state2
            .executed_end_event_counter
            .insert("End".to_string(), 1);
        assert_eq!(next_states, vec![state1, state2]);
    }

    #[test]
    fn try_execute_terminate_end_event() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/terminate_end.bpmn"));
        let process = get_process_by_id(&collaboration, "p1_process");

        let terminate_end_event: &FlowNode = get_flow_node_with_id(process, "end");
        let start_state = collaboration.create_start_state();

        let next_states = terminate_end_event.try_execute(
            get_snapshot_by_id(&start_state, "p1_process"),
            &start_state,
            &collaboration,
        );

        let mut expected_state = State {
            snapshots: vec![
                ProcessSnapshot::new("p0_process", vec!["flow3"]),
                ProcessSnapshot::new("p1_process", vec![]),
            ],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
        };
        expected_state
            .executed_end_event_counter
            .insert("end".to_string(), 1);
        assert_eq!(next_states, vec![expected_state]);
    }

    #[test]
    fn try_execute_intermediate_throw_event() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/intermediate_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "Intermediate");
        let start_state = State::new("process", vec!["Flow_1", "Flow_2"]);

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state,
            &collaboration,
        );

        assert_eq!(
            next_states,
            vec![
                State::new("process", vec!["Flow_2", "Flow_3", "Flow_4",],),
                State::new("process", vec!["Flow_1", "Flow_3", "Flow_4",],),
            ]
        );
    }

    #[test]
    fn try_execute_message_start() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/message_start_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, "start");
        let start_state = State {
            snapshots: vec![],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::from([(String::from("mf"), 1u16)]),
        };

        let next_states = flow_node.try_trigger_message_start_event(process, &start_state);

        assert_eq!(
            next_states,
            vec![State {
                snapshots: vec![ProcessSnapshot::new("p1_process", vec!["start_out"])],
                executed_end_event_counter: BTreeMap::new(),
                messages: BTreeMap::new(),
            }]
        );
    }

    fn get_flow_nodes_executed_to_reach(
        model_checking_result: &ModelCheckingResult,
        state_hash: u64,
    ) -> Vec<String> {
        let path_to_unsafe = model_checking_result
            .state_space
            .get_path_to_state(state_hash)
            .unwrap();
        get_flow_nodes_for_path(path_to_unsafe)
    }

    fn get_flow_nodes_for_path(path_to_unsafe: Vec<(String, u64)>) -> Vec<String> {
        path_to_unsafe
            .into_iter()
            .map(|(executed_flow_node_id, _)| executed_flow_node_id)
            .collect()
    }

    #[test]
    fn safeness_unfulfilled() {
        let collaboration =
            read_bpmn_and_unwrap(&(PATH.to_string() + "properties/safeness/unsafe.bpmn"));

        let model_checking_result = collaboration.explore_state_space(vec![Property::Safeness]);

        let unsafe_state_hash: u64 = 3842228032089975966;

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult {
                property: Property::Safeness,
                fulfilled: false,
                problematic_elements: vec![String::from("Unsafe2"), String::from("Unsafe1")],
                problematic_state_hashes: vec![10963063677454573590, unsafe_state_hash]
            }]
        );

        let unsafe_state = model_checking_result.get_state(&unsafe_state_hash).unwrap();
        assert_eq!(
            unsafe_state,
            &State {
                snapshots: vec![ProcessSnapshot {
                    id: "process",
                    tokens: BTreeMap::from([("Unsafe1", 2u16)]),
                }],
                executed_end_event_counter: BTreeMap::new(),
                messages: BTreeMap::new(),
            }
        );

        let path_to_unsafe =
            get_flow_nodes_executed_to_reach(&model_checking_result, unsafe_state_hash);

        assert_eq!(
            vec!["Gateway_0wc9tmn", "Gateway_0re1nqe", "Gateway_0re1nqe"],
            path_to_unsafe
        );
    }

    #[test]
    fn safeness_fulfilled() {
        let collaboration = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/pg.bpmn"));

        let model_checking_result = collaboration.explore_state_space(vec![Property::Safeness]);

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult::safe()]
        );
    }

    #[test]
    fn option_to_complete_unfulfilled_1() {
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/option_to_complete/no-option-to-complete-1.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::OptionToComplete]);

        let not_terminated_state_hash_1 = 6735018309777973944;
        let not_terminated_state_hash_2 = 9452229757242377755;
        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult {
                property: Property::OptionToComplete,
                fulfilled: false,
                problematic_state_hashes: vec![
                    not_terminated_state_hash_1,
                    not_terminated_state_hash_2,
                ],
                ..Default::default()
            }]
        );

        let path_to_not_terminated_1 =
            get_flow_nodes_executed_to_reach(&model_checking_result, not_terminated_state_hash_1);

        assert_eq!(
            vec!["Gateway_0do975f", "Activity_0x2nbu7"],
            path_to_not_terminated_1
        );

        let path_to_not_terminated_2 =
            get_flow_nodes_executed_to_reach(&model_checking_result, not_terminated_state_hash_2);

        assert_eq!(
            vec!["Gateway_0do975f", "Activity_03mx8x5"],
            path_to_not_terminated_2
        );
    }

    #[test]
    fn option_to_complete_unfulfilled_2() {
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/option_to_complete/no-option-to-complete-2.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::OptionToComplete]);

        let expected_hash: u64 = 12581154331755844142;

        assert_eq!(
            model_checking_result.property_results,
            vec![PropertyResult {
                property: Property::OptionToComplete,
                fulfilled: false,
                problematic_state_hashes: vec![expected_hash],
                ..Default::default()
            }]
        );

        let stuck_state = model_checking_result.get_state(&expected_hash).unwrap();
        assert_eq!(
            stuck_state,
            &State {
                snapshots: vec![ProcessSnapshot {
                    id: "Process_dc137d1f-9555-4446-bfd0-adebe6a3bdb2",
                    tokens: BTreeMap::from([("stuck", 1u16)]),
                }],
                executed_end_event_counter: BTreeMap::new(),
                messages: BTreeMap::new(),
            }
        );
    }

    #[test]
    fn option_to_complete_fulfilled() {
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
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/no-proper-completion-1.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::ProperCompletion]);

        let result = model_checking_result.property_results.first().unwrap();
        assert_eq!(
            result,
            &PropertyResult {
                property: Property::ProperCompletion,
                fulfilled: false,
                problematic_elements: vec!["EndEvent_1".to_string()],
                problematic_state_hashes: vec![12782631182175227902],
            }
        );
    }

    #[test]
    fn proper_completion_unfulfilled_2() {
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/no-proper-completion-2.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::ProperCompletion]);

        let result = model_checking_result.property_results.first().unwrap();
        assert_eq!(
            result,
            &PropertyResult {
                property: Property::ProperCompletion,
                fulfilled: false,
                problematic_elements: vec!["EndEvent_1".to_string()],
                problematic_state_hashes: vec![12782631182175227902],
            }
        );
    }

    #[test]
    fn proper_completion_unfulfilled_3() {
        let collaboration = read_bpmn_and_unwrap(
            &(PATH.to_string() + "properties/proper_completion/no-proper-completion-3-unsafe.bpmn"),
        );

        let model_checking_result =
            collaboration.explore_state_space(vec![Property::Safeness, Property::ProperCompletion]);

        let result = model_checking_result.property_results.get(1).unwrap();
        assert_eq!(
            result,
            &PropertyResult {
                property: Property::ProperCompletion,
                fulfilled: false,
                problematic_elements: vec!["EndEvent_1".to_string()],
                problematic_state_hashes: vec![5271536939354034460],
            }
        );
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

    fn get_snapshot_by_id<'a>(start_state: &'a State, id: &str) -> &'a ProcessSnapshot<'a> {
        start_state
            .snapshots
            .iter()
            .find(|snapshot| snapshot.id == id)
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
