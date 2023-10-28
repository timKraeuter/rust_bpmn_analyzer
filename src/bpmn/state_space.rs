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
#[derive(Debug)]
pub struct Token {
    // Element id of the element the token is located at.
    pub position: String
}