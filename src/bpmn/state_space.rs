pub struct StateSpace {
    states: Vec<State>
}

pub struct State {
    states: Vec<ProcessSnapshot>
}

pub struct ProcessSnapshot {
    tokens: Vec<Token>
}

pub struct Token {
    // Element id of the element the token is located at.
    position: String
}