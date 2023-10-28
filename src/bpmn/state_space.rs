#[derive(Debug)]
pub struct StateSpace {
    pub states: Vec<State>
    // Transitions are currently missing here. They could be handled in each individual state.
}
#[derive(Debug)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>
}
#[derive(Debug)]
pub struct ProcessSnapshot {
    pub tokens: Vec<Token>
}


impl Clone for ProcessSnapshot {
    fn clone(&self) -> Self {
        ProcessSnapshot {
            tokens: self.tokens.to_vec()
        }
    }
}
#[derive(Debug, Clone)]
pub struct Token {
    // Element id of the element the token is located at.
    pub position: String
}