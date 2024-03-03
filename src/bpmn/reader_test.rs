#[cfg(test)]
mod tests {
    use crate::bpmn::collaboration::Collaboration;
    use crate::bpmn::flow_node::{EventType, FlowNode};
    use crate::bpmn::flow_node::{FlowNodeType, SequenceFlow};
    use crate::bpmn::process::Process;
    use crate::bpmn::reader::read_bpmn_file;

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

    fn get_flow_node_ids(process: &Process) -> Vec<String> {
        process.flow_nodes.iter().map(|f| f.id.clone()).collect()
    }

    #[test]
    fn read_task_and_gateways() {
        let mut expected = Collaboration {
            name: String::from("task-and-gateways.bpmn"),
            participants: Vec::new(),
        };
        let mut process = Process {
            id: String::from("process_id"),
            flow_nodes: Vec::new(),
        };
        process.add_flow_node(FlowNode::new(
            String::from("start"),
            FlowNodeType::StartEvent(EventType::None),
        ));
        process.add_flow_node(FlowNode::new(String::from("task"), FlowNodeType::Task));
        process.add_flow_node(FlowNode::new(
            String::from("exg"),
            FlowNodeType::ExclusiveGateway,
        ));
        process.add_flow_node(FlowNode::new(
            String::from("pg"),
            FlowNodeType::ParallelGateway,
        ));
        process.add_flow_node(FlowNode::new(
            String::from("end"),
            FlowNodeType::EndEvent(EventType::None),
        ));
        process.add_sf(
            SequenceFlow {
                id: String::from("sf_1"),
            },
            String::from("start"),
            String::from("task"),
        );
        process.add_sf(
            SequenceFlow {
                id: String::from("sf_2"),
            },
            String::from("task"),
            String::from("exg"),
        );
        process.add_sf(
            SequenceFlow {
                id: String::from("sf_3"),
            },
            String::from("exg"),
            String::from("pg"),
        );
        process.add_sf(
            SequenceFlow {
                id: String::from("sf_4"),
            },
            String::from("pg"),
            String::from("end"),
        );
        expected.add_participant(process);

        // When
        let result = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/task-and-gateways.bpmn"));

        assert_eq!(expected, result);
    }

    #[test]
    fn read_different_namespace_prefixes() {
        let result1 =
            read_bpmn_and_unwrap(&String::from(&(PATH.to_string() + "prefix/no-prefix.bpmn")));

        assert_eq!("no-prefix.bpmn", result1.name);
        let first_participant = result1.participants.first().unwrap();
        assert_eq!(5, first_participant.flow_nodes.len());

        let result2 = read_bpmn_and_unwrap(&String::from(
            &(PATH.to_string() + "prefix/bpmn-prefix.bpmn"),
        ));

        assert_eq!("bpmn-prefix.bpmn", result2.name);
        let first_participant = result2.participants.first().unwrap();
        assert_eq!(10, first_participant.flow_nodes.len());

        let result3 = read_bpmn_and_unwrap(&String::from(
            &(PATH.to_string() + "prefix/wurst-prefix.bpmn"),
        ));

        assert_eq!("wurst-prefix.bpmn", result3.name);
        let first_participant = result3.participants.first().unwrap();
        assert_eq!(10, first_participant.flow_nodes.len());
    }

    #[test]
    fn read_pools_and_messages() {
        let collaboration = read_bpmn_and_unwrap(&String::from(
            &(PATH.to_string() + "reader/pools-message-flows.bpmn"),
        ));

        assert_eq!("pools-message-flows.bpmn", collaboration.name);
        let first_participant = collaboration
            .participants
            .iter()
            .find(|p| p.id == "p1_process")
            .unwrap();
        assert_eq!(4, first_participant.flow_nodes.len());
        let flow_node_ids = get_flow_node_ids(first_participant);
        assert_eq!(
            vec!["startP1", "sendEvent", "SendTask", "endP1"],
            flow_node_ids
        );

        let second_participant = collaboration
            .participants
            .iter()
            .find(|p| p.id == "p2_process")
            .unwrap();
        assert_eq!(4, second_participant.flow_nodes.len());
        let flow_node_ids = get_flow_node_ids(second_participant);
        assert_eq!(
            vec!["startP2", "receiveEvent", "ReceiveTask", "endP2"],
            flow_node_ids
        );
    }

    #[test]
    fn read_all_possible_tasks() {
        let result = read_bpmn_file(&String::from(&(PATH.to_string() + "reader/tasks.bpmn")));

        match result {
            Ok(_) => {
                panic!("This should be an error")
            }
            Err(err) => {
                assert_eq!(vec!["call_activity".to_string()], err.unsupported_elements);
            }
        }
    }

    #[test]
    fn read_all_possible_events() {
        let result = read_bpmn_file(&String::from(&(PATH.to_string() + "reader/events.bpmn")));

        match result {
            Ok(_) => {
                panic!("This should be an error")
            }
            Err(err) => {
                assert_eq!(
                    vec![
                        "signalStart".to_string(),
                        "signalEnd".to_string(),
                        "terminateEnd".to_string(),
                        "linkCEvent".to_string(),
                        "linkTEvent".to_string(),
                        "signalCEvent".to_string(),
                        "signalTEvent".to_string(),
                        "timerCEvent".to_string(),
                        "errorEnd".to_string(),
                        "escalationEnd".to_string(),
                        "escalationTEvent".to_string(),
                        "compensationTEvent".to_string(),
                        "compensationEnd".to_string(),
                        "timerStart".to_string(),
                    ],
                    err.unsupported_elements
                );
            }
        }
    }

    #[test]
    fn read_all_possible_gateways() {
        let result = read_bpmn_file(&String::from(&(PATH.to_string() + "reader/gateways.bpmn")));

        match result {
            Ok(_) => {
                panic!("This should be an error")
            }
            Err(err) => {
                assert_eq!(
                    vec![
                        "event_gateway".to_string(),
                        "inclusive_gateway".to_string(),
                        "instantiate_event_gateway".to_string(),
                        "complex_gateway".to_string(),
                    ],
                    err.unsupported_elements
                );
            }
        }
    }

    #[test]
    fn read_event_subprocess() {
        let result = read_bpmn_file(&String::from(
            &(PATH.to_string() + "reader/event-subprocesses.bpmn"),
        ));

        match result {
            Ok(_) => {
                panic!("This should be an error")
            }
            Err(err) => {
                assert_eq!(
                    vec![
                        "Event_subprocess1".to_string(),
                        "signalNon".to_string(),
                        "signal".to_string(),
                        "Event_subprocess2".to_string(),
                        "esc".to_string(),
                        "escNon".to_string(),
                        "error".to_string(),
                    ],
                    err.unsupported_elements
                );
            }
        }
    }
}
