#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::bpmn::{BPMNCollaboration, BPMNProcess, FlowNode, GeneralProperty, GeneralPropertyResult, read_bpmn_file};
    use crate::bpmn::state_space::{ProcessSnapshot, State};

    #[test]
    fn create_start_state() {
        let collaboration = read_bpmn_file(&String::from("test/resources/start.bpmn"));

        let start_state = collaboration.create_start_state();

        assert_eq!(start_state, State {
            snapshots: vec![ProcessSnapshot::new(
                String::from("process"),
                vec![String::from("Flow_1"), String::from("Flow_2")])],
        });
    }

    #[test]
    fn try_execute_task() {
        let collaboration = read_bpmn_file(&String::from("test/resources/task.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Activity_A"));
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_2"), String::from("Flow_3"), String::from("Flow_4")]),
            State::new(String::from("process"), vec![String::from("Flow_1"), String::from("Flow_3"), String::from("Flow_4")]),
        ])
    }

    #[test]
    fn try_execute_exg_choice() {
        let collaboration = read_bpmn_file(&String::from("test/resources/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_1"));
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_2")]),
            State::new(String::from("process"), vec![String::from("Flow_3")]),
        ])
    }

    #[test]
    fn try_execute_exg_merge() {
        let collaboration = read_bpmn_file(&String::from("test/resources/exg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_2"));
        let start_state = State::new(String::from("process"), vec![
            String::from("Flow_2"),
            String::from("Flow_3")],
        );

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_3"), String::from("Flow_4")]),
            State::new(String::from("process"), vec![String::from("Flow_2"), String::from("Flow_4")]),
        ]);
    }

    #[test]
    fn try_execute_pg_split() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_1"));
        let start_state = collaboration.create_start_state();

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_2"), String::from("Flow_3")]),
        ])
    }

    #[test]
    fn try_execute_pg_sync() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Gateway_2"));
        let start_state = State::new(String::from("process"), vec![
            String::from("Flow_2"),
            String::from("Flow_3")],
        );

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_4")]),
        ]);
    }

    #[test]
    fn try_execute_end_event() {
        let collaboration = read_bpmn_file(&String::from("test/resources/end.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("End"));
        let start_state = State::new(String::from("process"), vec![
            String::from("Flow_1"),
            String::from("Flow_1"),
            String::from("Flow_2")],
        );

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_1"), String::from("Flow_2")]),
            State::new(String::from("process"), vec![String::from("Flow_1"), String::from("Flow_1")]),
        ]);
    }

    #[test]
    fn try_execute_intermediate_throw_event() {
        let collaboration = read_bpmn_file(&String::from("test/resources/intermediate_event.bpmn"));
        let process = get_first_process(&collaboration);

        let flow_node: &FlowNode = get_flow_node_with_id(process, String::from("Intermediate"));
        let start_state = State::new(String::from("process"), vec![
            String::from("Flow_1"),
            String::from("Flow_2")],
        );

        let next_states = flow_node.try_execute(
            get_first_snapshot(&start_state),
            &start_state);

        assert_eq!(next_states, vec![
            State::new(String::from("process"), vec![String::from("Flow_2"), String::from("Flow_3"), String::from("Flow_4")]),
            State::new(String::from("process"), vec![String::from("Flow_1"), String::from("Flow_3"), String::from("Flow_4")]),
        ]);
    }

    #[test]
    fn safeness_property_unfulfilled() {
        let collaboration = read_bpmn_file(&String::from("test/resources/unsafe.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::Safeness],
        );

        assert_eq!(model_checking_result.property_results, vec![
            GeneralPropertyResult {
                property: GeneralProperty::Safeness,
                fulfilled: false,
                problematic_elements: vec![String::from("Unsafe")],
                problematic_state_hashes: vec![]
            }
        ]);
    }

    #[test]
    fn safeness_property_fulfilled() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::Safeness],
        );

        assert_eq!(model_checking_result.property_results, vec![GeneralPropertyResult::safe()]);
    }

    #[test]
    fn option_to_complete_property_unfulfilled_1() {
        let collaboration = read_bpmn_file(&String::from("test/resources/option_to_complete/no-option-to-complete-1.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::OptionToComplete],
        );

        assert_eq!(model_checking_result.property_results, vec![GeneralPropertyResult {
            property: GeneralProperty::OptionToComplete,
            fulfilled: false,
            problematic_state_hashes: vec![2865282549678524369, 14709088705232714226],
            problematic_elements: vec![]
        }]);
    }

    #[test]
    fn option_to_complete_property_unfulfilled_2() {
        let collaboration = read_bpmn_file(&String::from("test/resources/option_to_complete/no-option-to-complete-2.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::OptionToComplete],
        );

        let expected_hash: u64 = 5226340431746374588;

        assert_eq!(model_checking_result.property_results, vec![GeneralPropertyResult {
            property: GeneralProperty::OptionToComplete,
            fulfilled: false,
            problematic_state_hashes: vec![expected_hash],
            problematic_elements: vec![]
        }]);

        let stuck_state = model_checking_result.state_space.states.get(&expected_hash).unwrap();
        assert_eq!(stuck_state, &State {
            snapshots: vec![ProcessSnapshot {
                id: String::from("Process_dc137d1f-9555-4446-bfd0-adebe6a3bdb2"),
                tokens: HashMap::from([(String::from("stuck"), 1i16)]),
            }]
        });
    }

    #[test]
    fn option_to_complete_property_fulfilled() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::OptionToComplete],
        );

        assert_eq!(model_checking_result.property_results, vec![GeneralPropertyResult::always_terminates()]);
    }

    #[test]
    fn no_dead_activities_property_unfulfilled() {
        let collaboration = read_bpmn_file(&String::from("test/resources/no_dead_activities/dead-activities.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::NoDeadActivities],
        );

        assert_eq!(model_checking_result.property_results, vec![GeneralPropertyResult {
            property: GeneralProperty::NoDeadActivities,
            fulfilled: false,
            problematic_elements: vec![String::from("123"), String::from("1234")],
            problematic_state_hashes: vec![]
        }]);
    }

    #[test]
    fn no_dead_activities_property_fulfilled() {
        let collaboration = read_bpmn_file(&String::from("test/resources/no_dead_activities/no-dead-activities.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::NoDeadActivities],
        );

        assert_eq!(model_checking_result.property_results, vec![GeneralPropertyResult::no_dead_activities()]);
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
        process.flow_nodes.iter().find(|flow_node| { flow_node.id == id }).unwrap()
    }
}