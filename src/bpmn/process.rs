use crate::bpmn::flow_node::{FlowNode, SequenceFlow};

#[derive(Debug, PartialEq)]
pub struct Process {
    pub id: String,
    pub flow_nodes: Vec<FlowNode>,
}

impl Process {
    fn find_flow_node(&mut self, id: &str) -> Option<&mut FlowNode> {
        self.flow_nodes.iter_mut().find(|f| f.id == id)
    }
    pub fn add_sf(&mut self, id: String, source_ref: &str, target_ref: &str) {
        let source = self.find_flow_node(source_ref);
        // TODO: Clone for now but maybe refactor using lifetimes?
        let id_clone = id.clone();

        match source {
            None => {
                panic!("There should be a flow node for the id \"{}\"", source_ref)
            }
            Some(source) => source.add_outgoing_flow(SequenceFlow { id }),
        }
        let target = self.find_flow_node(target_ref);
        match target {
            None => {
                panic!("There should be a flow node for the id \"{}\"", target_ref)
            }
            Some(target) => target.add_incoming_flow(SequenceFlow { id: id_clone }),
        }
    }
    pub fn add_flow_node(&mut self, flow_node: FlowNode) {
        self.flow_nodes.push(flow_node);
    }
}
