use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType, SequenceFlow, TaskType};
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
    let mut mfs = vec![];
    let mut unsupported_elements = vec![];
    let mut last_event_start_bytes: Option<BytesStart> = None;
    let mut last_event_type: Option<EventType> = None;
    let mut current_participant = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.local_name().as_ref() {
                b"process" => {
                    add_participant(&mut collaboration, &e);
                    current_participant =
                        Some(get_attribute_value_or_panic(&e, &String::from("id")));
                }
                b"subProcess" => {
                    if has_true_attribute_value(&e, "triggeredByEvent") {
                        // Event subprocesses not supported.
                        unsupported_elements.push(e);
                    } else {
                        // Normal subprocesses are currently handled as a task. (We assume they finish)
                        add_flow_node(
                            &mut collaboration,
                            &e,
                            FlowNodeType::Task(TaskType::Default),
                        )
                    }
                }
                b"task" | b"sendTask" | b"serviceTask" | b"userTask" | b"manualTask"
                | b"businessRuleTask" | b"scriptTask" => add_flow_node(
                    &mut collaboration,
                    &e,
                    FlowNodeType::Task(TaskType::Default),
                ),
                b"receiveTask" => add_flow_node(
                    &mut collaboration,
                    &e,
                    FlowNodeType::Task(TaskType::Receive),
                ),
                b"startEvent"
                | b"intermediateCatchEvent"
                | b"intermediateThrowEvent"
                | b"endEvent" => {
                    last_event_start_bytes = Some(e);
                }
                b"parallelGateway" => {
                    add_flow_node(&mut collaboration, &e, FlowNodeType::ParallelGateway)
                }
                b"exclusiveGateway" => {
                    add_flow_node(&mut collaboration, &e, FlowNodeType::ExclusiveGateway)
                }
                b"eventBasedGateway" => {
                    add_flow_node(&mut collaboration, &e, FlowNodeType::EventBasedGateway)
                }
                b"sequenceFlow" => {
                    sfs.push(e);
                }
                b"messageFlow" => {
                    mfs.push(e);
                }
                b"callActivity" | b"inclusiveGateway" | b"complexGateway" => {
                    unsupported_elements.push(e)
                }
                _ => (),
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"process" => {
                    if unsupported_elements.is_empty() {
                        sfs.iter().for_each(|sf| match &current_participant {
                            None => {
                                panic!("Sequence flow found but no BPMN process! Malformed XML?")
                            }
                            Some(current_participant) => {
                                add_sf_to_participant(&mut collaboration, sf, current_participant)
                            }
                        });
                    }
                    sfs = vec![];
                }
                b"startEvent"
                | b"intermediateCatchEvent"
                | b"intermediateThrowEvent"
                | b"endEvent" => {
                    let last_event_bytes = last_event_start_bytes.unwrap();
                    let event_type = last_event_type.unwrap_or(EventType::None);
                    if event_type == EventType::Unsupported {
                        unsupported_elements.push(last_event_bytes);
                    } else {
                        add_event(&mut collaboration, &last_event_bytes, event_type);
                    }
                    last_event_start_bytes = None;
                    last_event_type = None;
                }
                _ => (),
            },
            Ok(Event::Empty(e)) => match e.local_name().as_ref() {
                b"sequenceFlow" => {
                    sfs.push(e);
                }
                b"messageFlow" => {
                    mfs.push(e);
                }
                b"messageEventDefinition" => {
                    last_event_type = Some(EventType::Message);
                }
                b"terminateEventDefinition" => {
                    last_event_type = Some(EventType::Terminate);
                }
                b"linkEventDefinition"
                | b"signalEventDefinition"
                | b"timerEventDefinition"
                | b"escalationEventDefinition"
                | b"errorEventDefinition"
                | b"compensateEventDefinition" => {
                    last_event_type = Some(EventType::Unsupported);
                }
                b"task" | b"sendTask" | b"serviceTask" | b"userTask" | b"manualTask"
                | b"businessRuleTask" | b"scriptTask" => add_flow_node(
                    &mut collaboration,
                    &e,
                    FlowNodeType::Task(TaskType::Default),
                ),
                b"receiveTask" => add_flow_node(
                    &mut collaboration,
                    &e,
                    FlowNodeType::Task(TaskType::Receive),
                ),
                b"eventBasedGateway" => {
                    add_flow_node(&mut collaboration, &e, FlowNodeType::EventBasedGateway)
                }
                b"callActivity" | b"inclusiveGateway" | b"complexGateway" => {
                    unsupported_elements.push(e)
                }
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
    for mf in mfs.into_iter() {
        collaboration.add_message_flow(
            get_attribute_value_or_panic(&mf, &String::from("id")),
            get_attribute_value_or_panic(&mf, &String::from("sourceRef")),
            get_attribute_value_or_panic(&mf, &String::from("targetRef")),
        );
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

fn add_participant(collaboration: &mut Collaboration, p_bytes: &BytesStart) {
    let id = get_attribute_value_or_panic(p_bytes, &String::from("id"));
    collaboration.add_participant(Process {
        id,
        flow_nodes: Vec::new(),
    });
}

fn add_event(
    collaboration: &mut Collaboration,
    flow_node_bytes: &BytesStart,
    event_type: EventType,
) {
    let event_type = match flow_node_bytes.local_name().as_ref() {
        b"startEvent" => FlowNodeType::StartEvent(event_type),
        b"intermediateCatchEvent" => FlowNodeType::IntermediateCatchEvent(event_type),
        b"intermediateThrowEvent" => FlowNodeType::IntermediateThrowEvent(event_type),
        b"endEvent" => FlowNodeType::EndEvent(event_type),
        _ => panic!("Should not happen!"),
    };
    add_flow_node(collaboration, flow_node_bytes, event_type);
}

fn add_flow_node(
    collaboration: &mut Collaboration,
    flow_node_bytes: &BytesStart,
    flow_node_type: FlowNodeType,
) {
    let id = get_attribute_value_or_panic(flow_node_bytes, &String::from("id"));
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

fn add_sf_to_participant(
    collaboration: &mut Collaboration,
    sf_bytes: &BytesStart,
    participant_id: &String,
) {
    let id = get_attribute_value_or_panic(sf_bytes, "id");
    let source_ref = get_attribute_value_or_panic(sf_bytes, "sourceRef");
    let target_ref = get_attribute_value_or_panic(sf_bytes, "targetRef");

    let sf = SequenceFlow { id };

    let process = collaboration
        .participants
        .iter_mut()
        .find(|p| p.id == *participant_id);
    match process {
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

fn has_true_attribute_value(e: &BytesStart, key: &str) -> bool {
    match e.try_get_attribute(key) {
        Ok(attribute) => match attribute {
            None => false,
            Some(x) => x.value.as_ref() == b"true",
        },
        Err(_) => false,
    }
}
