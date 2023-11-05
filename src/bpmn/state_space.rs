use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct StateSpace {
    pub states: HashMap<u64, State>,
    // Outgoing transitions (using hashes to satisfy the borrow checker for now)
    pub transitions: HashMap<u64, Vec<u64>>

}
#[derive(Debug, Hash, PartialEq)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>
}
#[derive(Debug, Hash, PartialEq)]
pub struct Transition (String, State);

impl State {
    pub fn new(snapshot_id: String, token_positions: Vec<String>) -> State {
        State {
            snapshots: vec![ProcessSnapshot::new(snapshot_id, token_positions)],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessSnapshot {
    pub id: String,
    pub tokens: HashMap<String, i16>
}

impl Hash for ProcessSnapshot {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
        for position_and_amount in self.tokens.iter() {
            position_and_amount.hash(hasher);
        }
    }
}

impl ProcessSnapshot {
    pub fn new(id: String, token_positions: Vec<String>) -> ProcessSnapshot {
        let mut snapshot = ProcessSnapshot {
            id,
            tokens: HashMap::new(),
        };
        for position in token_positions {
            snapshot.add_token(position);
        }
        snapshot
    }
    pub fn add_token(&mut self, position: String) {
        match self.tokens.get(&position) {
            None => { self.tokens.insert(position, 1) }
            Some(amount) => { self.tokens.insert(position, amount + 1) }
        };
    }
    pub fn remove_token(&mut self, position: String) {
        match self.tokens.get(&position) {
            None => {panic!("Token {} should be removed but was not present!", position)}
            Some(amount) => {
                let new_amount = amount - 1;
                if new_amount == 0 {
                    self.tokens.remove(&position);
                } else {
                    self.tokens.insert(position, new_amount);
                }
            }
        }
    }
}