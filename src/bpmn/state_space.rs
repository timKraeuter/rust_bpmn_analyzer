#[derive(Debug)]
pub struct StateSpace {
    pub states: Vec<State>
    // Transitions are currently missing here. They could be handled in each individual state.
}
#[derive(Debug, Hash, PartialEq)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>
}
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct ProcessSnapshot {
    pub id: String,
    pub tokens: Vec<Token>
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Token {
    // Element id of the element the token is located at.
    pub position: String
}