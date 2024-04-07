use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType, TaskType};
use crate::bpmn::process::Process;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::fmt;

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

pub fn read_bpmn_string(
    contents: &str,
    file_name: String,
) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    let mut reader = Reader::from_str(contents);
    reader.trim_text(true);

    let mut collaboration = Collaboration {
        name: file_name,
        participants: vec![],
    };

    let mut sfs = vec![];
    let mut mfs = vec![];
    let mut unsupported_elements = vec![];
    let mut last_event_start_bytes: Option<BytesStart> = None;
    let mut last_event_type: Option<EventType> = Some(EventType::None);
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
                    match last_event_type {
                        None => {
                            unsupported_elements.push(last_event_bytes);
                        }
                        Some(event_type) => {
                            add_event(&mut collaboration, &last_event_bytes, event_type);
                        }
                    }
                    last_event_start_bytes = None;
                    last_event_type = Some(EventType::None);
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
                b"linkEventDefinition" => {
                    let mut link_name = get_attribute_value(&e, "name");
                    if link_name.is_none() {
                        // Fallback to link event name.
                        if let Some(link_event) = last_event_start_bytes.as_ref() {
                            link_name = get_attribute_value(link_event, "name");
                        }
                    }
                    last_event_type = Some(EventType::Link(link_name.unwrap_or_default()));
                }
                b"signalEventDefinition"
                | b"timerEventDefinition"
                | b"escalationEventDefinition"
                | b"errorEventDefinition"
                | b"compensateEventDefinition" => {
                    last_event_type = None; // Set to none since these are unsupported.
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
            get_attribute_value_or_panic(&mf, "id"),
            get_attribute_value_or_panic(&mf, "sourceRef"),
            get_attribute_value_or_panic(&mf, "targetRef"),
        );
    }
    Ok(collaboration)
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

    let process = collaboration
        .participants
        .iter_mut()
        .find(|p| p.id == *participant_id);
    match process {
        None => {
            panic!("Sequence flow found but no BPMN process! Malformed XML?")
        }
        Some(process) => {
            process.add_sf(id, source_ref.as_str(), target_ref.as_str());
        }
    }
}

fn get_attribute_value_or_panic(e: &BytesStart, key: &str) -> String {
    match get_attribute_value(e, key) {
        None => {
            panic!("Attribute value for key \"{}\" not found in {:?}.", key, e)
        }
        Some(value) => value,
    }
}

fn get_attribute_value(e: &BytesStart, key: &str) -> Option<String> {
    match e.try_get_attribute(key) {
        Ok(attribute) => match attribute {
            None => None,
            Some(x) => Some(
                String::from_utf8(x.value.into_owned())
                    .unwrap_or_else(|e| panic!("UTF8 Error. {}", e)),
            ),
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
