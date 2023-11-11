use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{FlowNode, FlowNodeType, SequenceFlow};
use crate::bpmn::process::Process;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::fs;
use std::path::Path;

pub fn read_bpmn_file(file_path: &String) -> Collaboration {
    // TODO: Read directly from file (less peak memory usage).
    // TODO: Use serde to map to structs.
    let (contents, file_name) = read_file_and_get_name(file_path);
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let mut collaboration = Collaboration {
        name: file_name,
        participants: Vec::new(),
    };

    let mut sfs = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"process" | b"bpmn:process" => {
                    add_participant(&mut collaboration, e);
                }
                b"startEvent" | b"bpmn:startEvent" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::StartEvent)
                }
                b"serviceTask" | b"bpmn:serviceTask" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::Task)
                }
                b"task" | b"bpmn:task" => add_flow_node(&mut collaboration, e, FlowNodeType::Task),
                b"intermediateThrowEvent" | b"bpmn:intermediateThrowEvent" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::IntermediateThrowEvent)
                }
                b"parallelGateway" | b"bpmn:parallelGateway" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::ParallelGateway)
                }
                b"exclusiveGateway" | b"bpmn:exclusiveGateway" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::ExclusiveGateway)
                }
                b"endEvent" | b"bpmn:endEvent" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::EndEvent)
                }
                b"sequenceFlow" | b"bpmn:sequenceFlow" => sfs.push(e),
                _ => (),
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"sequenceFlow" | b"bpmn:sequenceFlow" => sfs.push(e),
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    // Read sfs at the end.
    for sf in sfs.iter() {
        add_sf_to_last_participant(&mut collaboration, sf);
    }
    collaboration
}

fn read_file_and_get_name(path: &String) -> (String, String) {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading the file {:?}. {}", path, err)
        }
    };
    (file_content, get_file_name(path))
}

fn get_file_name(path: &String) -> String {
    let path = Path::new(path);
    // Wtf is the next line. Careful file might not exist!
    path.file_name().unwrap().to_str().unwrap().parse().unwrap()
}

fn add_participant(collaboration: &mut Collaboration, p_bytes: BytesStart) {
    let id = get_attribute_value_or_panic(&p_bytes, &String::from("id"));
    collaboration.add_participant(Process {
        id,
        flow_nodes: Vec::new(),
    });
}

fn add_flow_node(
    collaboration: &mut Collaboration,
    flow_node_bytes: BytesStart,
    flow_node_type: FlowNodeType,
) {
    let id = get_attribute_value_or_panic(&flow_node_bytes, &String::from("id"));
    let option = collaboration.participants.last_mut();
    match option {
        None => {
            panic!("Sequence flow found but no BPMN process! Malformed XML?")
        }
        Some(process) => {
            process.add_flow_node(FlowNode::new(id, flow_node_type));
        }
    }
}

fn add_sf_to_last_participant(collaboration: &mut Collaboration, sf_bytes: &BytesStart) {
    let id = get_attribute_value_or_panic(sf_bytes, &String::from("id"));
    let source_ref = get_attribute_value_or_panic(sf_bytes, &String::from("sourceRef"));
    let target_ref = get_attribute_value_or_panic(sf_bytes, &String::from("targetRef"));
    let sf = SequenceFlow { id };

    let option = collaboration.participants.last_mut();
    match option {
        None => {
            panic!("Sequence flow found but no BPMN process! Malformed XML?")
        }
        Some(process) => {
            process.add_sf(sf, source_ref, target_ref);
        }
    }
}

fn get_attribute_value_or_panic(e: &BytesStart, key: &str) -> String {
    match e.try_get_attribute(key) {
        Ok(attribute) => match attribute {
            None => {
                panic!("Attribute value for key \"{}\" not found.", key)
            }
            Some(x) => match String::from_utf8(x.value.into_owned()) {
                Ok(value) => value,
                Err(e) => {
                    panic!("UTF8 Error. {}", e)
                }
            },
        },
        Err(e) => {
            panic!("Could not get attribute! {}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bpmn::flow_node::FlowNode;

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
        let result = read_bpmn_file(&String::from("tests/resources/task-and-gateways.bpmn"));

        assert_eq!(expected, result);
    }
}
