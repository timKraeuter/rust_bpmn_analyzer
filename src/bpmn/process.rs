use crate::bpmn::flow_node::{FlowNode, SequenceFlow};

#[derive(Debug, PartialEq)]
pub struct Process {
    pub id: String,
    pub flow_nodes: Vec<FlowNode>,
}

impl Process {
    fn find_flow_node_idx_by_id(&self, id: &str) -> Option<usize> {
        self.flow_nodes.iter().position(|f| f.id == id)
    }

    pub fn add_sf(&mut self, id: String, source_ref: &str, target_ref: &str) {
        let source_idx = self.find_flow_node_idx_by_id(source_ref);
        let target_idx = self.find_flow_node_idx_by_id(target_ref);
        match (source_idx, target_idx) {
            (Some(source_idx), Some(target_idx)) => {
                let source = self.flow_nodes.get_mut(source_idx).unwrap();
                source.add_outgoing_flow(SequenceFlow {
                    id: id.clone(),
                    source_idx,
                    target_idx,
                });

                let target = self.flow_nodes.get_mut(target_idx).unwrap();
                target.add_incoming_flow(SequenceFlow {
                    id,
                    source_idx,
                    target_idx,
                });
            }
            (_, _) => {
                panic!(
                    "There should be flow nodes for the ids \"{}\" and \"{}\".",
                    source_ref, target_ref
                )
            }
        }
    }

    pub fn add_flow_node(&mut self, flow_node: FlowNode) {
        self.flow_nodes.push(flow_node);
    }
}
