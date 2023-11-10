use crate::bpmn::flow_node::{FlowNode, SequenceFlow};

#[derive(Debug, PartialEq)]
pub struct Process {
    pub id: String,
    pub flow_nodes: Vec<FlowNode>,
}

impl Process {
    pub fn add_sf(&mut self, sf: SequenceFlow, source_ref: String, target_ref: String) {
        let mut source_flow_node: Vec<&mut FlowNode> = self
            .flow_nodes
            .iter_mut()
            .filter(|f| f.id == source_ref)
            .collect();
        // TODO: Clone for now but maybe refactor using lifetimes?
        let sf_id = sf.id.clone();

        match source_flow_node.last_mut() {
            None => {
                panic!("There should be a flow node for the id \"{}\"", source_ref)
            }
            Some(source) => source.add_outgoing_flow(sf),
        }
        let mut target_flow_node: Vec<&mut FlowNode> = self
            .flow_nodes
            .iter_mut()
            .filter(|f| f.id == target_ref)
            .collect();
        match target_flow_node.last_mut() {
            None => {
                panic!("There should be a flow node for the id \"{}\"", target_ref)
            }
            Some(target) => target.add_incoming_flow(SequenceFlow { id: sf_id }),
        }
    }
    pub fn add_flow_node(&mut self, flow_node: FlowNode) {
        self.flow_nodes.push(flow_node);
    }
}
