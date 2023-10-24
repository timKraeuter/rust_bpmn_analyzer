pub struct BPMNCollaboration {
    name: String,
    participants: Vec<BPMNProcess>
}

pub struct BPMNProcess {
    name: String,
    sequence_flows: Vec<SequenceFlow>,
    flow_nodes: Vec<FlowNode>
}

pub struct SequenceFlow {
    name: String
}

pub struct FlowNode {
    name: String
}