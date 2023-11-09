#[cfg(test)]
mod tests {
    use crate::bpmn::state_space::{ProcessSnapshot, State};
    use crate::bpmn::{
        read_bpmn_file, BPMNCollaboration, BPMNProcess, FlowNode, GeneralProperty,
        GeneralPropertyResult, ModelCheckingResult,
    };
    use std::collections::HashMap;

    #[test]
    fn create_start_state() {
        let collaboration = read_bpmn_file(&String::from("test/resources/start.bpmn"));

        let start_state = collaboration.create_start_state();

        assert_eq!(
            start_state,
            State {
                snapshots: vec![ProcessSnapshot::new(
                    String::from("process"),
                    vec![String::from("Flow_1"), String::from("Flow_2")]
                )],
            }
        );
    }

    #[test]
    fn try_execute_task() {
        let collaboration = read_bpmn_file(&String::from("test/resources/task.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Activity_A"));
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![
                State::new(
                    String::from("process"),
                    vec![
                        String::from("Flow_2"),
                        String::from("Flow_3"),
                        String::from("Flow_4")
                    ]
                ),
                State::new(
                    String::from("process"),
                    vec![
                        String::from("Flow_1"),
                        String::from("Flow_3"),
                        String::from("Flow_4")
                    ]
                ),
            ]
        )
    }

    #[test]
    fn try_execute_exg_choice() {
        let collaboration = read_bpmn_file(&String::from("test/resources/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_1"));
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![
                State::new(String::from("process"), vec![String::from("Flow_2")]),
                State::new(String::from("process"), vec![String::from("Flow_3")]),
            ]
        )
    }

    #[test]
    fn try_execute_exg_merge() {
        let collaboration = read_bpmn_file(&String::from("test/resources/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_2"));
        let start_state = State::new(
            String::from("process"),
            vec![String::from("Flow_2"), String::from("Flow_3")],
        );

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![
                State::new(
                    String::from("process"),
                    vec![String::from("Flow_3"), String::from("Flow_4")]
                ),
                State::new(
                    String::from("process"),
                    vec![String::from("Flow_2"), String::from("Flow_4")]
                ),
            ]
        );
    }

    #[test]
    fn try_execute_pg_split() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_1"));
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![State::new(
                String::from("process"),
                vec![String::from("Flow_2"), String::from("Flow_3")]
            ),]
        )
    }

    #[test]
    fn try_execute_pg_sync() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_2"));
        let start_state = State::new(
            String::from("process"),
            vec![String::from("Flow_2"), String::from("Flow_3")],
        );

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![State::new(
                String::from("process"),
                vec![String::from("Flow_4")]
            ),]
        );
    }

    #[test]
    fn try_execute_end_event() {
        let collaboration = read_bpmn_file(&String::from("test/resources/end.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("End"));
        let start_state = State::new(
            String::from("process"),
            vec![
                String::from("Flow_1"),
                String::from("Flow_1"),
                String::from("Flow_2"),
            ],
        );

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![
                State::new(
                    String::from("process"),
                    vec![String::from("Flow_1"), String::from("Flow_2")]
                ),
                State::new(
                    String::from("process"),
                    vec![String::from("Flow_1"), String::from("Flow_1")]
                ),
            ]
        );
    }

    #[test]
    fn try_execute_intermediate_throw_event() {
        let collaboration = read_bpmn_file(&String::from("test/resources/intermediate_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Intermediate"));
        let start_state = State::new(
            String::from("process"),
            vec![String::from("Flow_1"), String::from("Flow_2")],
        );

        let next_states = flow_node.try_execute(get_first_snapshot(&start_state), &start_state);

        assert_eq!(
            next_states,
            vec![
                State::new(
                    String::from("process"),
                    vec![
                        String::from("Flow_2"),
                        String::from("Flow_3"),
                        String::from("Flow_4")
                    ]
                ),
                State::new(
                    String::from("process"),
                    vec![
                        String::from("Flow_1"),
                        String::from("Flow_3"),
                        String::from("Flow_4")
                    ]
                ),
            ]
        );
    }

    fn get_flow_nodes_executed_to_reach(
        model_checking_result: &ModelCheckingResult,
        state_hash: u64,
    ) -> Vec<String> {
        let path_to_unsafe = model_checking_result.get_path_to_state(state_hash).unwrap();
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
        let collaboration = read_bpmn_file(&String::from("test/resources/unsafe.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::Safeness]);

        let unsafe_state_hash: u64 = 7943842223099901633;

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult {
                property: GeneralProperty::Safeness,
                fulfilled: false,
                problematic_elements: vec![String::from("Unsafe")],
                problematic_state_hashes: vec![unsafe_state_hash],
                ..Default::default()
            }]
        );

        let unsafe_state = model_checking_result.get_state(&unsafe_state_hash).unwrap();
        assert_eq!(
            unsafe_state,
            &State {
                snapshots: vec![ProcessSnapshot {
                    id: String::from("process"),
                    tokens: HashMap::from([(String::from("Unsafe"), 2u16)]),
                }]
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
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::Safeness]);

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult::safe()]
        );
    }

    #[test]
    fn option_to_complete_unfulfilled_1() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/option_to_complete/no-option-to-complete-1.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::OptionToComplete]);

        let not_terminated_state_hash_1 = 2865282549678524369;
        let not_terminated_state_hash_2 = 14709088705232714226;
        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult {
                property: GeneralProperty::OptionToComplete,
                fulfilled: false,
                problematic_state_hashes: vec![
                    not_terminated_state_hash_1,
                    not_terminated_state_hash_2
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
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/option_to_complete/no-option-to-complete-2.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::OptionToComplete]);

        let expected_hash: u64 = 5226340431746374588;

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult {
                property: GeneralProperty::OptionToComplete,
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
                    id: String::from("Process_dc137d1f-9555-4446-bfd0-adebe6a3bdb2"),
                    tokens: HashMap::from([(String::from("stuck"), 1u16)]),
                }]
            }
        );
    }

    #[test]
    fn option_to_complete_fulfilled() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::OptionToComplete]);

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult::always_terminates()]
        );
    }

    #[test]
    fn no_dead_activities_unfulfilled() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/no_dead_activities/dead-activities.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::NoDeadActivities]);

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult {
                property: GeneralProperty::NoDeadActivities,
                fulfilled: false,
                problematic_elements: vec![
                    String::from("Dead_1"),
                    String::from("Dead_2"),
                    String::from("Dead_3")
                ],
                ..Default::default()
            }]
        );
    }

    #[test]
    fn no_dead_activities_fulfilled() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/no_dead_activities/no-dead-activities.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::NoDeadActivities]);

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult::no_dead_activities()]
        );
    }

    #[test]
    fn proper_completion_fulfilled_1() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/proper_completion/proper-completion-1.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::ProperCompletion]);

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult::proper_completion()]
        );
    }

    #[test]
    fn proper_completion_fulfilled_2() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/proper_completion/proper-completion-2.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::ProperCompletion]);

        assert_eq!(
            model_checking_result.property_results,
            vec![GeneralPropertyResult::proper_completion()]
        );
    }

    #[test]
    fn proper_completion_unfulfilled_1() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/proper_completion/no-proper-completion-1.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::ProperCompletion]);

        let result = model_checking_result.property_results.get(0).unwrap();
        assert!(!result.fulfilled);
        assert_eq!(result.property, GeneralProperty::ProperCompletion);
        assert_eq!(
            result.problematic_elements,
            vec![String::from("EndEvent_1")]
        );

        // Same path but different hashes due to hashmap ordering of tokens.
        let expected_path_1 = vec![
            ("Gateway_043ppqt".to_string(), 11544654538002244535),
            ("EndEvent_1".to_string(), 15019738428683846599),
            ("EndEvent_1".to_string(), 3001297282923415647),
        ];
        let expected_path_2 = vec![
            ("Gateway_043ppqt".to_string(), 11468766902515292417),
            ("EndEvent_1".to_string(), 15019738428683846599),
            ("EndEvent_1".to_string(), 3001297282923415647),
        ];

        assert!(
            result.counter_example == expected_path_1 || result.counter_example == expected_path_2
        );
    }

    #[test]
    fn proper_completion_unfulfilled_2() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/proper_completion/no-proper-completion-2.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result =
            collaboration.explore_state_space(start, vec![GeneralProperty::ProperCompletion]);

        let result = model_checking_result.property_results.get(0).unwrap();
        assert!(!result.fulfilled);
        assert_eq!(result.property, GeneralProperty::ProperCompletion);
        assert_eq!(
            result.problematic_elements,
            vec![String::from("EndEvent_1")]
        );
    }

    #[test]
    fn proper_completion_unfulfilled_3() {
        let collaboration = read_bpmn_file(&String::from(
            "test/resources/proper_completion/no-proper-completion-3-unsafe.bpmn",
        ));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::Safeness, GeneralProperty::ProperCompletion],
        );

        let result = model_checking_result.property_results.get(1).unwrap();
        assert!(!result.fulfilled);
        assert_eq!(result.property, GeneralProperty::ProperCompletion);
        assert_eq!(
            result.problematic_elements,
            vec![String::from("EndEvent_1")]
        );
    }

    fn get_first_process(collaboration: &BPMNCollaboration) -> &BPMNProcess {
        let process = collaboration.participants.get(0).unwrap();
        process
    }

    fn get_first_snapshot(start_state: &State) -> &ProcessSnapshot {
        let snapshot = start_state.snapshots.get(0).unwrap();
        snapshot
    }

    fn get_flow_node_with_id(process: &BPMNProcess, id: String) -> &FlowNode {
        process
            .flow_nodes
            .iter()
            .find(|flow_node| flow_node.id == id)
            .unwrap()
    }
}
