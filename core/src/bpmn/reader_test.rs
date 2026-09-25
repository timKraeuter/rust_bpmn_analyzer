#[cfg(test)]
mod tests {
    use crate::bpmn::collaboration::Collaboration;
    use crate::bpmn::flow_node::FlowNodeType;
    use crate::bpmn::flow_node::{EventType, FlowNode, TaskType};
    use crate::bpmn::process::Process;
    use crate::bpmn::reader::{read_bpmn_from_file, read_bpmn_from_string};
    use rstest::rstest;
    use std::collections::HashMap;

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

    fn get_flow_node_ids(process: &Process) -> Vec<&str> {
        process.flow_nodes.iter().map(|f| f.id.as_str()).collect()
    }

    fn find_flow_node_by_id<'a>(p: &'a Process, id: &str) -> &'a FlowNode {
        p.flow_nodes
            .iter()
            .find(|f| f.id == id)
            .unwrap_or_else(|| panic!("Flow node with id {} not found", id))
    }

    #[test]
    fn read_task_and_gateways() {
        let mut expected = Collaboration {
            participants: Vec::new(),
        };
        let mut process = Process {
            id: String::from("process_id"),
            flow_nodes: vec![],
            sequence_flow_index: HashMap::new(),
        };
        process.add_flow_node(FlowNode::new(
            String::from("start"),
            FlowNodeType::StartEvent(EventType::None),
        ));
        process.add_flow_node(FlowNode::new(
            String::from("task"),
            FlowNodeType::Task(TaskType::Default),
        ));
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
        process.add_sf(String::from("sf_1"), "start", "task");
        process.add_sf(String::from("sf_2"), "task", "exg");
        process.add_sf(String::from("sf_3"), "exg", "pg");
        process.add_sf(String::from("sf_4"), "pg", "end");
        expected.add_participant(process);

        // When
        let result = read_bpmn_and_unwrap(&(PATH.to_string() + "semantics/task_and_gateways.bpmn"));

        assert_eq!(expected, result);
    }

    #[test]
    fn read_different_namespace_prefixes() {
        let result1 =
            read_bpmn_and_unwrap(&String::from(&(PATH.to_string() + "prefix/no-prefix.bpmn")));

        let first_participant = result1.participants.first().unwrap();
        assert_eq!(5, first_participant.flow_nodes.len());

        let result2 = read_bpmn_and_unwrap(&String::from(
            &(PATH.to_string() + "prefix/bpmn-prefix.bpmn"),
        ));

        let first_participant = result2.participants.first().unwrap();
        assert_eq!(10, first_participant.flow_nodes.len());

        let result3 = read_bpmn_and_unwrap(&String::from(
            &(PATH.to_string() + "prefix/wurst-prefix.bpmn"),
        ));

        let first_participant = result3.participants.first().unwrap();
        assert_eq!(10, first_participant.flow_nodes.len());
    }

    #[test]
    fn read_pools_and_messages() {
        let collaboration = read_bpmn_and_unwrap(&String::from(
            &(PATH.to_string() + "reader/pools-message-flows.bpmn"),
        ));

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
        // Check message flows
        let send_event = find_flow_node_by_id(first_participant, "sendEvent");
        assert_eq!(
            FlowNodeType::IntermediateThrowEvent(EventType::Message),
            send_event.flow_node_type
        );
        assert_eq!(1, send_event.outgoing_message_flows.len());
        let send_task = find_flow_node_by_id(first_participant, "SendTask");
        assert_eq!(1, send_task.outgoing_message_flows.len());
        let end_event = find_flow_node_by_id(first_participant, "endP1");
        assert_eq!(
            FlowNodeType::EndEvent(EventType::Message),
            end_event.flow_node_type
        );
        assert_eq!(1, end_event.outgoing_message_flows.len());

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
        // Check message flows
        let start_event = find_flow_node_by_id(second_participant, "startP2");
        assert_eq!(
            FlowNodeType::StartEvent(EventType::Message),
            start_event.flow_node_type
        );
        assert_eq!(1, start_event.incoming_message_flows.len());
        let receive_event = find_flow_node_by_id(second_participant, "receiveEvent");
        assert_eq!(
            FlowNodeType::IntermediateCatchEvent(EventType::Message),
            receive_event.flow_node_type
        );
        assert_eq!(1, receive_event.incoming_message_flows.len());
        let receive_task = find_flow_node_by_id(second_participant, "ReceiveTask");
        assert_eq!(1, receive_task.incoming_message_flows.len());
    }

    #[test]
    fn read_all_possible_tasks() {
        let result = read_bpmn_from_file(&String::from(&(PATH.to_string() + "reader/tasks.bpmn")));

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
        let result = read_bpmn_from_file(&String::from(&(PATH.to_string() + "reader/events.bpmn")));

        match result {
            Ok(_) => {
                panic!("This should be an error")
            }
            Err(err) => {
                assert_eq!(
                    vec![
                        "signalStart".to_string(),
                        "signalEnd".to_string(),
                        "signalCEvent".to_string(),
                        "signalTEvent".to_string(),
                        "timerCEvent".to_string(),
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

    #[rstest]
    #[case("<errorEventDefinition/>")]
    #[case("<errorEventDefinition errorRef=\"error\"/>")]
    #[case("<errorEventDefinition errorRef=\"error\"></errorEventDefinition>")]
    fn read_error_end_event(#[case] definition: &str) {
        let xml = format!(
            r#"<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
                <error id="error" errorCode="failure"/>
                <process id="process">
                    <startEvent id="start"></startEvent>
                    <endEvent id="errorEnd">{definition}</endEvent>
                    <endEvent id="normalEnd"></endEvent>
                    <sequenceFlow id="flow" sourceRef="start" targetRef="errorEnd"/>
                </process>
            </definitions>"#
        );
        let collaboration = read_bpmn_from_string(&xml).unwrap();
        let process = &collaboration.participants[0];
        let error_end = find_flow_node_by_id(process, "errorEnd");
        assert_eq!(error_end.incoming_flows[0].id, "flow");
        assert_eq!(
            error_end.flow_node_type,
            FlowNodeType::EndEvent(EventType::Error)
        );
        assert_eq!(
            find_flow_node_by_id(process, "normalEnd").flow_node_type,
            FlowNodeType::EndEvent(EventType::None)
        );
    }

    #[rstest]
    #[case("startEvent", "<errorEventDefinition/>")]
    #[case("intermediateCatchEvent", "<errorEventDefinition/>")]
    #[case("intermediateThrowEvent", "<errorEventDefinition/>")]
    #[case("boundaryEvent", "<errorEventDefinition/>")]
    #[case("boundaryEvent", "<errorEventDefinition></errorEventDefinition>")]
    fn reject_unsupported_error_catches_and_throws(#[case] event: &str, #[case] definition: &str) {
        let xml = format!(
            r#"<definitions><process id="process">
                <task id="task"/>
                <{event} id="unsupported" attachedToRef="task">{definition}</{event}>
                <endEvent id="normalEnd"></endEvent>
            </process></definitions>"#
        );
        let error = read_bpmn_from_string(&xml).unwrap_err();
        assert_eq!(error.unsupported_elements, vec!["unsupported"]);
    }

    #[rstest]
    #[case("subProcess")]
    #[case("transaction")]
    #[case("adHocSubProcess")]
    fn reject_nested_error_end_event(#[case] subprocess: &str) {
        let xml = format!(
            r#"<definitions><process id="process">
                <startEvent id="start"></startEvent>
                <{subprocess} id="subprocess">
                    <endEvent id="nestedError"><errorEventDefinition/></endEvent>
                </{subprocess}>
                <sequenceFlow id="flow" sourceRef="start" targetRef="subprocess"/>
            </process></definitions>"#
        );
        let error = read_bpmn_from_string(&xml).unwrap_err();
        assert_eq!(error.unsupported_elements, vec!["nestedError"]);
    }

    #[test]
    fn read_all_possible_gateways() {
        let result =
            read_bpmn_from_file(&String::from(&(PATH.to_string() + "reader/gateways.bpmn")));

        match result {
            Ok(_) => {
                panic!("This should be an error")
            }
            Err(err) => {
                assert_eq!(
                    vec![
                        "inclusive_gateway".to_string(),
                        "complex_gateway".to_string(),
                    ],
                    err.unsupported_elements
                );
            }
        }
    }

    #[test]
    fn read_event_subprocess() {
        let result = read_bpmn_from_file(&String::from(
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
