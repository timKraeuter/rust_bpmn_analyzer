use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct StateSpace {
    pub start_state_hash: u64,
    pub terminated_state_hashes: Vec<u64>,
    pub states: HashMap<u64, State>,
    // Outgoing transitions (executed flow node id, target state hash)
    pub transitions: HashMap<u64, Vec<(String, u64)>>,
}

impl StateSpace {
    pub fn mark_terminated_if_needed(&mut self, state: &State, state_hash: u64) {
        if state.is_terminated() {
            self.terminated_state_hashes.push(state_hash);
        }
    }

    pub fn get_state(&self, state_hash: &u64) -> &State {
        match self.states.get(state_hash) {
            None => {
                panic!("State for {} not found!", state_hash)
            }
            Some(state) => state,
        }
    }

    // TODO: This can loop and never end
    pub fn get_path_to_state(&self, state_hash: u64) -> Option<Vec<(String, u64)>> {
        self.get_path(self.start_state_hash, state_hash)
    }
    fn get_path(&self, from_state_hash: u64, to_state_hash: u64) -> Option<Vec<(String, u64)>> {
        match self.transitions.get(&from_state_hash) {
            None => None,
            Some(next_states) => {
                match next_states
                    .iter()
                    .find(|(_, next_state_hash)| next_state_hash == &to_state_hash)
                {
                    None => {}
                    Some(last_transition) => {
                        return Some(vec![last_transition.clone()]);
                    }
                }
                // Not found continue searching
                for (flow_node_id, next_state) in next_states {
                    match self.get_path(*next_state, to_state_hash) {
                        None => {}
                        Some(mut path) => {
                            path.insert(0, (flow_node_id.clone(), *next_state));
                            return Some(path);
                        }
                    };
                }
                None
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Serialize, Clone)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>,
    pub executed_end_event_counter: BTreeMap<String, u16>,
}

impl State {
    pub fn new(snapshot_id: String, tokens: Vec<String>) -> State {
        State {
            snapshots: vec![ProcessSnapshot::new(snapshot_id, tokens)],
            executed_end_event_counter: BTreeMap::new(),
        }
    }

    pub fn calc_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn is_terminated(&self) -> bool {
        self.snapshots
            .iter()
            .all(|snapshot| snapshot.tokens.is_empty())
    }

    pub fn get_unsafe_sf(&self) -> Option<&String> {
        const TWO: u16 = 2;
        for snapshot in self.snapshots.iter() {
            match snapshot.tokens.iter().find(|(_, amount)| *amount >= &TWO) {
                None => {}
                Some((sf, _)) => {
                    return Some(sf);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize)]
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
