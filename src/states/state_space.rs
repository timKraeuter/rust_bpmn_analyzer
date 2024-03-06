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
        self.states
            .get(state_hash)
            .unwrap_or_else(|| panic!("State for {} not found!", state_hash))
    }

    pub fn get_path_to_state(&self, state_hash: u64) -> Option<Vec<(String, u64)>> {
        if self.start_state_hash == state_hash {
            return Some(vec![]);
        }
        self.get_path(self.start_state_hash, state_hash, &mut HashMap::new())
    }
    fn get_path(
        &self,
        from_state_hash: u64,
        to_state_hash: u64,
        seen_states: &mut HashMap<u64, bool>,
    ) -> Option<Vec<(String, u64)>> {
        match self.transitions.get(&from_state_hash) {
            None => None,
            Some(next_states) => {
                for (flow_node_id, next_state_hash) in next_states {
                    if seen_states.contains_key(next_state_hash) {
                        continue;
                    }
                    if *next_state_hash == to_state_hash {
                        return Some(vec![(flow_node_id.clone(), *next_state_hash)]);
                    }
                    seen_states.insert(*next_state_hash, true);
                    match self.get_path(*next_state_hash, to_state_hash, seen_states) {
                        None => {}
                        Some(mut path) => {
                            path.insert(0, (flow_node_id.clone(), *next_state_hash));
                            return Some(path);
                        }
                    };
                }
                None
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq)]
pub struct State {
    pub snapshots: Vec<ProcessSnapshot>,
    pub messages: BTreeMap<String, u16>,
    pub executed_end_event_counter: BTreeMap<String, u16>,
}

impl State {
    pub fn new(snapshot_id: String, tokens: Vec<&str>) -> State {
        State {
            snapshots: vec![ProcessSnapshot::new(snapshot_id, tokens)],
            executed_end_event_counter: BTreeMap::new(),
            messages: BTreeMap::new(),
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

    pub fn try_find_unsafe_sf_id(&self) -> Option<&String> {
        self.snapshots.iter().find_map(|snapshot| {
            snapshot.tokens.iter().find_map(
                |(sf_id, amount)| {
                    if *amount >= 2u16 {
                        Some(sf_id)
                    } else {
                        None
                    }
                },
            )
        })
    }

    pub fn add_message(&mut self, position: &str) {
        let count = self.messages.get_mut(position);
        if let Some(count) = count {
            *count += 1;
        } else {
            self.messages.insert(position.to_string(), 1);
        }
    }
    pub fn delete_message(&mut self, position: &str) {
        match self.messages.get_mut(position) {
            None => {
                panic!(
                    "Message {} should be removed but was not present!",
                    position
                )
            }
            Some(amount) => {
                *amount -= 1;
                if *amount == 0 {
                    self.messages.remove(position);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ProcessSnapshot {
    pub id: String,
    pub tokens: BTreeMap<String, u16>,
}

impl ProcessSnapshot {
    pub fn new(id: String, tokens: Vec<&str>) -> ProcessSnapshot {
        let mut snapshot = ProcessSnapshot {
            id,
            tokens: BTreeMap::new(),
        };
        for position in tokens {
            snapshot.add_token(position);
        }
        snapshot
    }

    pub fn add_token(&mut self, position: &str) {
        let count = self.tokens.get_mut(position);
        if let Some(count) = count {
            *count += 1;
        } else {
            self.tokens.insert(position.to_string(), 1);
        }
    }
    pub fn delete_token(&mut self, position: &str) {
        match self.tokens.get_mut(position) {
            None => {
                panic!("Token {} should be removed but was not present!", position)
            }
            Some(amount) => {
                *amount -= 1;
                if *amount == 0 {
                    self.tokens.remove(position);
                }
            }
        }
    }
}
