#[derive(Debug)]
pub struct BPMNCollaboration {
    pub name: String,
    pub participants: Vec<BPMNProcess>
}

impl BPMNCollaboration {
    pub fn add_participant(&mut self, participant: BPMNProcess) {
        self.participants.push(participant);
    }
}

#[derive(Debug)]
pub struct BPMNProcess {
    pub id: String,
    pub sequence_flows: Vec<SequenceFlow>,
    pub flow_nodes: Vec<FlowNode>
}

impl BPMNProcess {
    pub fn add_sf(&mut self, sf: SequenceFlow) {
        self.sequence_flows.push(sf);
    }
    pub fn add_flow_node(&mut self, flow_node: FlowNode) {
        self.flow_nodes.push(flow_node);
    }
}

#[derive(Debug)]
pub struct SequenceFlow {
    pub id: String
}
#[derive(Debug)]
pub struct  FlowNode {
    pub id: String,
    pub flow_node_type: FlowNodeType
}
#[derive(Debug)]
pub enum FlowNodeType {
    StartEvent,
    Task,
    ExclusiveGateway,
    ParallelGateway
}