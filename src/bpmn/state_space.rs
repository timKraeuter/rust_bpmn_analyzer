#[derive(Debug)]
pub struct StateSpace {
    pub states: Vec<State>
    // Transitions are currently missing here. They could be handled in each individual state.
}
#[derive(Debug, Hash, PartialEq)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>
}

impl State {
    pub(crate) fn new(snapshot_id: String, token_positions: Vec<String>) -> State {
        State {
            snapshots: vec![ProcessSnapshot::new(snapshot_id, token_positions)]
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct ProcessSnapshot {
    pub id: String,
    pub tokens: Vec<Token>

}

impl ProcessSnapshot {
    pub(crate) fn new(id: String, token_positions: Vec<String>) -> ProcessSnapshot {
        ProcessSnapshot {
            id,
            tokens: token_positions.into_iter().map(|token_position| {Token {
                position: token_position
            }}).collect()
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Token {
    // Element id of the element the token is located at.
    pub position: String
}