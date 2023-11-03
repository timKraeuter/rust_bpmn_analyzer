#[cfg(test)]
mod tests {
    use crate::bpmn::{BPMNCollaboration, BPMNProcess, FlowNode, GeneralProperty, GeneralPropertyResult, read_bpmn_file};
    use crate::bpmn::state_space::{ProcessSnapshot, State};

    #[test]
    fn create_start_state() {
        let collaboration = read_bpmn_file(&String::from("test/resources/start.bpmn"));

        let start_state = collaboration.create_start_state();

        assert_eq!(start_state, State {
            snapshots: vec![ProcessSnapshot::new(
                String::from("process"),
                vec![String::from("Flow_1"), String::from("Flow_2")])]
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
    fn safeness_property_unsafe() {
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
            }
        ]);
    }

    #[test]
    fn safeness_property_safe() {
        let collaboration = read_bpmn_file(&String::from("test/resources/pg.bpmn"));

        let start = collaboration.create_start_state();
        let model_checking_result = collaboration.explore_state_space(
            start,
            vec![GeneralProperty::Safeness],
        );

        assert_eq!(model_checking_result.property_results, vec![]);
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