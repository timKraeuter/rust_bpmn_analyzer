use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType, SequenceFlow};
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
    let mut unsupported_elements = vec![];
    let mut last_event_start_bytes: Option<BytesStart> = None;
    let mut last_event_type: Option<EventType> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.local_name().as_ref() {
                b"process" => {
                    add_participant(&mut collaboration, e);
                }
                b"serviceTask" | b"userTask" | b"manualTask" | b"subProcess"
                | b"businessRuleTask" | b"scriptTask" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::Task)
                }
                b"task" => add_flow_node(&mut collaboration, e, FlowNodeType::Task),
                b"startEvent"
                | b"intermediateCatchEvent"
                | b"intermediateThrowEvent"
                | b"endEvent" => {
                    last_event_start_bytes = Some(e);
                }
                b"parallelGateway" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::ParallelGateway)
                }
                b"exclusiveGateway" => {
                    add_flow_node(&mut collaboration, e, FlowNodeType::ExclusiveGateway)
                }
                b"sequenceFlow" => sfs.push(e),
                b"sendTask" | b"receiveTask" | b"callActivity" | b"eventBasedGateway"
                | b"inclusiveGateway" | b"complexGateway" => unsupported_elements.push(e),
                _ => (),
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"startEvent"
                | b"intermediateCatchEvent"
                | b"intermediateThrowEvent"
                | b"endEvent" => {
                    let last_event_bytes = last_event_start_bytes.unwrap();
                    let event_type = last_event_type.unwrap_or(EventType::None);
                    if event_type == EventType::Unsupported {
                        unsupported_elements.push(last_event_bytes);
                    } else {
                        add_event(&mut collaboration, last_event_bytes, event_type);
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
                b"sendTask" | b"receiveTask" | b"callActivity" | b"eventBasedGateway"
                | b"inclusiveGateway" | b"complexGateway" => unsupported_elements.push(e),
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

fn add_participant(collaboration: &mut Collaboration, p_bytes: BytesStart) {
    let id = get_attribute_value_or_panic(&p_bytes, &String::from("id"));
    collaboration.add_participant(Process {
        id,
        flow_nodes: Vec::new(),
    });
}

fn add_event(
    collaboration: &mut Collaboration,
    flow_node_bytes: BytesStart,
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
