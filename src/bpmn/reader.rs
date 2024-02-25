use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType, SequenceFlow};
use crate::bpmn::process::Process;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::path::Path;
use std::{fmt, fs};

#[derive(Debug)]
pub struct UnsupportedBpmnElementsError {
    pub unsupported_elements: Vec<String>,
}

impl fmt::Display for UnsupportedBpmnElementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unsupported BPMN elements found: {:?}",
            self.unsupported_elements
        )
    }
}

impl std::error::Error for UnsupportedBpmnElementsError {}

pub fn read_bpmn_file(file_path: &String) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    // TODO: Read directly from file (less peak memory usage).
    // TODO: Use serde to map to structs.
    let (contents, file_name) = read_file_and_get_name(file_path);
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let mut collaboration = Collaboration {
        name: file_name,
        participants: vec![],
    };

    let mut sfs = vec![];
    let mut unsupported_elements = vec![];
    let mut last_event_start_bytes: Option<BytesStart> = None;
    let mut last_event_type: Option<EventType> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.local_name().as_ref() {
                b"process" => {
                    add_participant(&mut collaboration, e);
                }
                b"startEvent" => add_flow_node(&mut collaboration, e, FlowNodeType::StartEvent),
                b"serviceTask" | b"userTask" | b"manualTask" | b"subProcess"
                | b"businessRuleTask" | b"scriptTask" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::Task)
                }
                b"task" => add_flow_node(&mut collaboration, e, FlowNodeType::Task),
                b"intermediateThrowEvent" | b"intermediateCatchEvent" | b"endEvent" => {
                    last_event_start_bytes = Some(e); // TODO: StartEvent must also be here soon.
                }
                b"parallelGateway" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::ParallelGateway)
                }
                b"exclusiveGateway" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::ExclusiveGateway)
                }
                b"sequenceFlow" => sfs.push(e),
                b"sendTask" | b"receiveTask" | b"callActivity" => unsupported_elements.push(e),
                _ => (),
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"startEvent" => {
                    last_event_type = None;
                }
                b"endEvent" => {
                    // TODO: Refactor this to a function together with the next cases.
                    let last_event_bytes = last_event_start_bytes.unwrap();
                    let event_type = last_event_type.unwrap_or(EventType::None);
                    if event_type == EventType::Unsupported {
                        unsupported_elements.push(last_event_bytes);
                    } else {
                        add_flow_node(
                            &mut collaboration,
                            last_event_bytes,
                            FlowNodeType::EndEvent(event_type),
                        );
                    }
                    last_event_start_bytes = None;
                    last_event_type = None;
                }
                b"intermediateThrowEvent" => {
                    let last_event_bytes = last_event_start_bytes.unwrap();
                    let event_type = last_event_type.unwrap_or(EventType::None);
                    if event_type == EventType::Unsupported {
                        unsupported_elements.push(last_event_bytes);
                    } else {
                        add_flow_node(
                            &mut collaboration,
                            last_event_bytes,
                            FlowNodeType::IntermediateThrowEvent(event_type),
                        );
                    }
                    last_event_start_bytes = None;
                    last_event_type = None;
                }
                b"intermediateCatchEvent" => {
                    let last_event_bytes = last_event_start_bytes.unwrap();
                    let event_type = last_event_type.unwrap_or(EventType::None);
                    if event_type == EventType::Unsupported {
                        unsupported_elements.push(last_event_bytes);
                    } else {
                        add_flow_node(
                            &mut collaboration,
                            last_event_bytes,
                            FlowNodeType::IntermediateCatchEvent(event_type),
                        );
                    }
                    last_event_start_bytes = None;
                    last_event_type = None;
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => match e.local_name().as_ref() {
                b"sequenceFlow" => sfs.push(e),
                b"messageEventDefinition" => {
                    last_event_type = Some(EventType::Message);
                }
                b"linkEventDefinition"
                | b"signalEventDefinition"
                | b"terminateEventDefinition"
                | b"timerEventDefinition"
                | b"escalationEventDefinition"
                | b"errorEventDefinition"
                | b"compensateEventDefinition" => {
                    last_event_type = Some(EventType::Unsupported);
                }
                b"task" => add_flow_node(&mut collaboration, e, FlowNodeType::Task),
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    if !unsupported_elements.is_empty() {
        let unsupported_elements: Vec<String> = unsupported_elements
            .iter()
            .map(|e| get_attribute_value_or_panic(e, &String::from("id")))
            .collect();
        return Err(UnsupportedBpmnElementsError {
            unsupported_elements,
        });
    }
    // Read sfs at the end.
    for sf in sfs.iter() {
        add_sf_to_last_participant(&mut collaboration, sf);
    }
    Ok(collaboration)
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
    let last_participant = collaboration.participants.last_mut();
    match last_participant {
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
                panic!("Attribute value for key \"{}\" not found in {:?}.", key, e)
            }
            Some(x) => String::from_utf8(x.value.into_owned())
                .unwrap_or_else(|e| panic!("UTF8 Error. {}", e)),
        },
        Err(e) => {
            panic!("Could not get attribute! {}", e)
        }
    }
}
