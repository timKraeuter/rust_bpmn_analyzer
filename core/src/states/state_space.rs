use crate::model_checking::properties::LiveLockError;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_TOKEN: u16 = 50;

pub static NEXT_SNAPSHOT_ID: AtomicUsize = AtomicUsize::new(0);

/// Reset the snapshot ID counter to 0. This is primarily used in tests.
pub fn reset_snapshot_counter() {
    NEXT_SNAPSHOT_ID.store(0, Ordering::Relaxed);
}

#[derive(Debug)]
pub struct StateSpace<'a> {
    pub start_state_hash: u64,
    pub terminated_state_hashes: Vec<u64>,
    pub states: HashMap<u64, State<'a>>,
    // Outgoing transitions (executed flow node id, target state hash)
    pub transitions: HashMap<u64, Vec<(&'a str, u64)>>,
}

impl StateSpace<'_> {
    pub fn mark_terminated_if_needed(&mut self, state: &State, state_hash: u64) {
        if state.is_terminated() {
            self.terminated_state_hashes.push(state_hash);
        }
    }

    pub fn get_state(&self, state_hash: &u64) -> &State<'_> {
        self.states
            .get(state_hash)
            .unwrap_or_else(|| panic!("State for {} not found!", state_hash))
    }

    pub fn get_path_to_state(&self, state_hash: u64) -> Option<Vec<(&str, u64)>> {
        if self.start_state_hash == state_hash {
            return Some(vec![]);
        }
        self.get_path(self.start_state_hash, state_hash, &mut HashMap::new())
            .map(|mut path| {
                path.reverse();
                path
            })
    }
    fn get_path(
        &self,
        from_state_hash: u64,
        to_state_hash: u64,
        seen_states: &mut HashMap<u64, bool>,
    ) -> Option<Vec<(&str, u64)>> {
        match self.transitions.get(&from_state_hash) {
            None => None,
            Some(next_states) => {
                for (flow_node_id, next_state_hash) in next_states {
                    if seen_states.contains_key(next_state_hash) {
                        continue;
                    }
                    if *next_state_hash == to_state_hash {
                        return Some(vec![(flow_node_id, *next_state_hash)]);
                    }
                    seen_states.insert(*next_state_hash, true);
                    match self.get_path(*next_state_hash, to_state_hash, seen_states) {
                        None => {}
                        Some(mut path) => {
                            path.push((flow_node_id, *next_state_hash));
                            return Some(path);
                        }
                    };
                }
                None
            }
        }
    }

    pub fn count_transitions(&self) -> usize {
        self.transitions
            .values()
            .flat_map(|targets| targets.iter())
            .count()
    }
}

#[derive(Debug, Hash, PartialEq)]
pub struct State<'a> {
    pub snapshots: Vec<ProcessSnapshot<'a>>,
    pub messages: BTreeMap<&'a str, u16>,
    pub executed_end_event_counter: BTreeMap<&'a str, u16>,
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "messages: {:?}, snapshots: {{ {} }}",
            self.messages,
            get_state_string(self)
        )
    }
}

fn get_state_string(state: &State) -> String {
    state
        .snapshots
        .iter()
        .map(|snapshot| format!("{}({}): {:?}", snapshot.process_id, snapshot.id, snapshot.tokens))
        .collect::<Vec<String>>()
        .join(", ")
}

impl<'a> State<'a> {
    pub fn new(snapshot_id: &'a str, tokens: Vec<&'a str>) -> State<'a> {
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

    pub fn find_unsafe_sf_ids_or_livelock(&self) -> Result<Vec<&str>, LiveLockError> {
        let mut result = vec![];
        for snapshot in self.snapshots.iter() {
            for (&sf_id, &amount) in snapshot.tokens.iter() {
                if amount >= MAX_TOKEN {
                    return Err(LiveLockError {
                        overflowing_position: sf_id.to_string(),
                    });
                }
                if amount >= 2u16 {
                    result.push(sf_id);
                }
            }
        }
        Ok(result)
    }

    pub fn add_message<'b>(&'b mut self, position: &'a str) {
        let count = self.messages.get_mut(position);
        if let Some(count) = count {
            *count += 1;
        } else {
            self.messages.insert(position, 1);
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

#[derive(Debug, Clone)]
pub struct ProcessSnapshot<'a> {
    pub id: usize,
    pub process_id: &'a str,
    pub tokens: BTreeMap<&'a str, u16>,
}

// Custom Hash implementation that excludes the id field
// This ensures that snapshots with the same process_id and tokens are considered equal
// regardless of their instance ID
impl<'a> Hash for ProcessSnapshot<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.process_id.hash(state);
        self.tokens.hash(state);
    }
}

// Custom PartialEq implementation that excludes the id field
// This must be consistent with Hash implementation
impl<'a> PartialEq for ProcessSnapshot<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.process_id == other.process_id && self.tokens == other.tokens
    }
}

impl<'a> ProcessSnapshot<'a> {
    pub fn new(process_id: &'a str, tokens: Vec<&'a str>) -> ProcessSnapshot<'a> {
        let id = NEXT_SNAPSHOT_ID.fetch_add(1, Ordering::Relaxed);
        let mut snapshot = ProcessSnapshot {
            id,
            process_id,
            tokens: BTreeMap::new(),
        };
        for position in tokens {
            snapshot.add_token(position);
        }
        snapshot
    }

    /// Create a ProcessSnapshot with a specific ID. Only for testing.
    #[cfg(test)]
    pub fn new_with_id(id: usize, process_id: &'a str, tokens: BTreeMap<&'a str, u16>) -> ProcessSnapshot<'a> {
        ProcessSnapshot {
            id,
            process_id,
            tokens,
        }
    }

    pub fn add_token(&mut self, position: &'a str) {
        let count = self.tokens.get_mut(position);
        if let Some(count) = count {
            *count += 1;
        } else {
            self.tokens.insert(position, 1);
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
