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
}

#[derive(Debug)]
pub struct SequenceFlow {
    pub id: String
}

#[derive(Debug)]
pub struct FlowNode {
    pub id: String
}