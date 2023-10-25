#[derive(Debug)]
pub struct BPMNCollaboration {
    pub name: String,
    pub participants: Vec<BPMNProcess>
}

#[derive(Debug)]
pub struct BPMNProcess {
    pub id: String,
    pub sequence_flows: Vec<SequenceFlow>,
    pub flow_nodes: Vec<FlowNode>
}

#[derive(Debug)]
pub struct SequenceFlow {
    pub id: String
}

#[derive(Debug)]
pub struct FlowNode {
    pub id: String
}