#[derive(Debug)]
pub struct StateSpace {
    pub states: Vec<State>
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