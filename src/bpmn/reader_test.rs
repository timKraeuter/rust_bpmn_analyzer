#[cfg(test)]
mod tests {
    use crate::bpmn::collaboration::Collaboration;
    use crate::bpmn::flow_node::FlowNode;
    use crate::bpmn::flow_node::{FlowNodeType, SequenceFlow};
    use crate::bpmn::process::Process;
    use crate::bpmn::reader::read_bpmn_file;

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
            FlowNodeType::StartEvent,
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
        process.add_flow_node(FlowNode::new(String::from("end"), FlowNodeType::EndEvent));
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
        let result = read_bpmn_file(&String::from(
            "tests/resources/unit/semantics/task-and-gateways.bpmn",
        ));

        assert_eq!(expected, result);
    }
}
