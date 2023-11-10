use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct StateSpace {
    pub start_state_hash: u64,
    pub terminated_state_hashes: Vec<u64>,
    pub states: HashMap<u64, State>,
    // Outgoing transitions (executed flow node id, target state hash)
    pub transitions: HashMap<u64, Vec<(String, u64)>>,
}

#[derive(Debug, Hash, PartialEq)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>,
}

impl State {
    pub fn new(snapshot_id: String, tokens: Vec<String>) -> State {
        State {
            snapshots: vec![ProcessSnapshot::new(snapshot_id, tokens)],
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.snapshots
            .iter()
            .all(|snapshot| snapshot.tokens.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ProcessSnapshot {
    pub id: String,
    pub tokens: BTreeMap<String, u16>,
}

impl ProcessSnapshot {
    pub fn new(id: String, tokens: Vec<String>) -> ProcessSnapshot {
        let mut snapshot = ProcessSnapshot {
            id,
            tokens: BTreeMap::new(),
        };
        for position in tokens {
            snapshot.add_token(position);
        }
        snapshot
    }
    pub fn add_token(&mut self, position: String) {
        match self.tokens.get(&position) {
            None => self.tokens.insert(position, 1),
            Some(amount) => self.tokens.insert(position, amount + 1),
        };
    }
    pub fn delete_token(&mut self, position: String) {
        match self.tokens.get(&position) {
            None => {
                panic!("Token {} should be removed but was not present!", position)
            }
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
