mod bpmn;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::str;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use crate::bpmn::{BPMNCollaboration, BPMNProcess, FlowNode, FlowNodeType, SequenceFlow};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<BPMNCollaboration, Box<dyn Error>> {
    // TODO: Read directly from file (less peak memory usage).
    // TODO: Use serde to map to structs.
    let (contents, file_name) = read_file_and_get_name(&config.file_path);
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let mut collaboration = BPMNCollaboration {
        name: file_name,
        participants: Vec::new(),
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"process" => {
                        add_participant(&mut collaboration, e);
                    }
                    b"startEvent" => add_flow_node_to_last_participant(&mut collaboration, e, FlowNodeType::StartEvent),
                    b"serviceTask" => add_flow_node_to_last_participant(&mut collaboration, e, FlowNodeType::Task),
                    b"parallelGateway" => add_flow_node_to_last_participant(&mut collaboration, e, FlowNodeType::ParallelGateway),
                    b"exclusiveGateway" => add_flow_node_to_last_participant(&mut collaboration, e, FlowNodeType::ExclusiveGateway),
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"sequenceFlow" => add_sf_to_last_participant(&mut collaboration, e),
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    Ok(collaboration)
}

fn read_file_and_get_name(path: &String) -> (String, String) {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => { content }
        Err(err) => { panic!("Error reading the file {:?}. {}", path, err) }
    };
    (file_content, get_file_name(path))
}

fn get_file_name(path: &String) -> String {
    let path = Path::new(path);
    // Wtf is the next line. Careful file might not exist!
    path.file_name().unwrap().to_str().unwrap().parse().unwrap()
}

fn add_participant(collaboration: &mut BPMNCollaboration, p_bytes: BytesStart) {
    let id = get_attribute_value_or_panic(p_bytes, &String::from("id"));
    collaboration.add_participant(BPMNProcess {
        id,
        sequence_flows: Vec::new(),
        flow_nodes: Vec::new(),
    });
}

fn add_flow_node_to_last_participant(collaboration: &mut BPMNCollaboration, flow_node_bytes: BytesStart, flow_node_type: FlowNodeType) {
    let id = get_attribute_value_or_panic(flow_node_bytes, &String::from("id"));
    let option = collaboration.participants.last_mut();
    match option {
        None => { panic!("Sequence flow found but no BPMN process! Malformed XML?") }
        Some(process) => {
            process.add_flow_node(FlowNode {
                id,
                flow_node_type,
            });
        }
    }
}

fn add_sf_to_last_participant(collaboration: &mut BPMNCollaboration, sf_bytes: BytesStart) {
    let id = get_attribute_value_or_panic(sf_bytes, &String::from("id"));
    let sf = SequenceFlow { id };

    let option = collaboration.participants.last_mut();
    match option {
        None => { panic!("Sequence flow found but no BPMN process! Malformed XML?") }
        Some(process) => {
            process.add_sf(sf);
        }
    }
}

pub fn get_attribute_value_or_panic(e: BytesStart, key: &str) -> String {
    match e.try_get_attribute(key) {
        Ok(x) => {
            match x {
                None => { panic!("Attribute value for key \"{}\" not found.", key) }
                Some(x) => {
                    match String::from_utf8(x.value.into_owned()) {
                        Ok(value) => { value }
                        Err(e) => { panic!("UTF8 Error. {}", e) }
                    }
                }
            }
        }
        Err(e) => { panic!("Could not get attribute! {}", e) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_task_and_gateways() {
        let mut expected = BPMNCollaboration { name: String::from("task-and-gateways.bpmn"), participants: Vec::new() };
        let mut process = BPMNProcess {
            id: String::from("process_id"),
            sequence_flows: Vec::new(),
            flow_nodes: Vec::new(),
        };
        process.add_sf(SequenceFlow {
            id: String::from("sf_1")
        });
        process.add_sf(SequenceFlow {
            id: String::from("sf_2")
        });
        process.add_sf(SequenceFlow {
            id: String::from("sf_3")
        });
        process.add_sf(SequenceFlow {
            id: String::from("sf_4")
        });
        process.add_flow_node(FlowNode {
            id: String::from("start"),
            flow_node_type: FlowNodeType::StartEvent
        });
        process.add_flow_node(FlowNode {
            id: String::from("task"),
            flow_node_type: FlowNodeType::Task
        });
        process.add_flow_node(FlowNode {
            id: String::from("exg"),
            flow_node_type: FlowNodeType::ExclusiveGateway
        });
        process.add_flow_node(FlowNode {
            id: String::from("pg"),
            flow_node_type: FlowNodeType::ParallelGateway
        });
        expected.add_participant(process);

        // When
        let result = run(Config { file_path: String::from("resources/task-and-gateways.bpmn") }).unwrap();

        assert_eq!(expected, result);
    }
}