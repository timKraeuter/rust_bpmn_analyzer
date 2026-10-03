//! Successor states described by the changes that executing a flow node makes to a state.
//!
//! Most successors found during state space exploration were reached before. A
//! [`SuccessorBuilder`] applies a [`StateChange`] to reusable buffers to calculate the hash of a
//! successor without cloning the current state, which is then only needed for new states. The
//! buffers hold ranks instead of ids, which are faster to compare.

use crate::bpmn::flow_node::{MessageFlow, SequenceFlow};
use crate::states::state_space::{ProcessSnapshot, State};
use rustc_hash::{FxHashMap, FxHasher};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

/// The changes that executing a flow node makes to a state.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct StateChange<'a> {
    /// Sequence flows that lose one token each.
    pub consumes_tokens: &'a [SequenceFlow],
    /// Whether all tokens of the executing snapshot are removed (terminate end events).
    pub consumes_all_tokens: bool,
    /// Sequence flows that receive one token each.
    pub produces_tokens: &'a [SequenceFlow],
    /// Message flow that loses one message.
    pub consumes_message: Option<&'a str>,
    /// Message flows that receive one message each.
    pub produces_messages: &'a [MessageFlow],
    /// End event whose execution is recorded.
    pub records_end_event: Option<&'a str>,
}

impl<'a> StateChange<'a> {
    /// Applies this change made by the given instance to the current state.
    ///
    /// # Returns
    /// The resulting successor.
    pub fn apply(&self, current_state: &State<'a>, instance: Instance<'_, 'a>) -> State<'a> {
        let mut snapshots: Vec<ProcessSnapshot<'a>> = current_state
            .snapshots
            .iter()
            .filter(|snapshot| instance.keeps(snapshot))
            .cloned()
            .collect();
        let mut changed_snapshot = ProcessSnapshot {
            id: instance.process_id(),
            tokens: BTreeMap::new(),
        };
        if let Instance::Existing(snapshot) = instance
            && !self.consumes_all_tokens
        {
            changed_snapshot.tokens = snapshot.tokens.clone();
        }
        for sf in self.consumes_tokens {
            changed_snapshot.delete_token(&sf.id);
        }
        for sf in self.produces_tokens {
            changed_snapshot.add_token(&sf.id);
        }
        snapshots.push(changed_snapshot);

        let mut successor = State {
            snapshots,
            messages: current_state.messages.clone(),
            executed_end_event_counter: current_state.executed_end_event_counter.clone(),
        };
        if let Some(mf) = self.consumes_message {
            successor.delete_message(mf);
        }
        for mf in self.produces_messages {
            successor.add_message(&mf.id);
        }
        if let Some(end_event) = self.records_end_event {
            *successor
                .executed_end_event_counter
                .entry(end_event)
                .or_insert(0) += 1;
        }
        successor
    }
}

/// The process instance executing a flow node.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Instance<'s, 'a> {
    /// A snapshot of the current state. The successor replaces all snapshots of its process with
    /// the changed snapshot, which is appended to the remaining snapshots.
    Existing(&'s ProcessSnapshot<'a>),
    /// A new instance of the process with this id, which a message start event starts. Its
    /// snapshot is appended to the snapshots of the current state.
    New(&'a str),
}

impl<'a> Instance<'_, 'a> {
    /// Returns whether the successor contains the given snapshot of the current state unchanged.
    fn keeps(&self, snapshot: &ProcessSnapshot) -> bool {
        match self {
            Instance::Existing(executing_snapshot) => snapshot.id != executing_snapshot.id,
            Instance::New(_) => true,
        }
    }

    fn process_id(&self) -> &'a str {
        match self {
            Instance::Existing(executing_snapshot) => executing_snapshot.id,
            Instance::New(process_id) => process_id,
        }
    }
}

/// Ranks of the ids that are keys of the maps of states, i.e., the ids of sequence flows, message
/// flows, and end events. Ranks are ordered like the ids.
#[derive(Debug)]
pub(crate) struct IdRanks<'a> {
    /// The ids in ascending order, i.e., the rank of an id is its index.
    ids: Vec<&'a str>,
    /// The ranks of the given ids by their address and length, which are faster to hash than ids.
    ranks: FxHashMap<*const str, usize>,
}

impl<'a> IdRanks<'a> {
    pub fn new(ids: impl IntoIterator<Item = &'a str>) -> IdRanks<'a> {
        let given_ids: Vec<&'a str> = ids.into_iter().collect();
        let mut ids = given_ids.clone();
        ids.sort_unstable();
        ids.dedup();
        let mut id_ranks = IdRanks {
            ids,
            ranks: FxHashMap::default(),
        };
        for id in given_ids {
            let rank = id_ranks.rank_by_content(id);
            id_ranks.ranks.insert(id, rank);
        }
        id_ranks
    }

    /// Returns the ids in ascending order, i.e., indexed by their rank.
    pub fn ids(&self) -> &[&'a str] {
        &self.ids
    }

    fn rank(&self, id: &str) -> usize {
        match self.ranks.get(&(id as *const str)) {
            Some(&rank) => rank,
            // The id has the content but not the address of a given id.
            None => self.rank_by_content(id),
        }
    }

    fn rank_by_content(&self, id: &str) -> usize {
        self.ids
            .binary_search(&id)
            .unwrap_or_else(|_| panic!("{} should have a rank but did not!", id))
    }
}

/// The process instance executing a flow node in the state loaded by a [`SuccessorBuilder`], see
/// [`Instance`].
#[derive(Debug, Clone, Copy)]
pub(crate) enum Executor<'a> {
    /// Index of the executing snapshot.
    Snapshot(usize),
    /// Process id of the instance started by a message start event.
    NewInstance(&'a str),
}

/// Applies changes to a state using reusable buffers.
#[derive(Debug)]
pub(crate) struct SuccessorBuilder<'r, 'a> {
    id_ranks: &'r IdRanks<'a>,
    /// Ranked tokens of each snapshot of the loaded state.
    snapshot_tokens: Vec<RankedCounter>,
    /// Ranked messages of the loaded state.
    messages: RankedCounter,
    /// Ranked executed end events of the loaded state.
    executed_end_event_counter: RankedCounter,
    successor_tokens: RankedCounter,
    successor_messages: RankedCounter,
    successor_executed_end_event_counter: RankedCounter,
}

impl<'r, 'a> SuccessorBuilder<'r, 'a> {
    pub fn new(id_ranks: &'r IdRanks<'a>) -> SuccessorBuilder<'r, 'a> {
        SuccessorBuilder {
            id_ranks,
            snapshot_tokens: vec![],
            messages: RankedCounter::default(),
            executed_end_event_counter: RankedCounter::default(),
            successor_tokens: RankedCounter::default(),
            successor_messages: RankedCounter::default(),
            successor_executed_end_event_counter: RankedCounter::default(),
        }
    }

    /// Loads the state to which changes are applied next.
    pub fn load(&mut self, state: &State<'a>) {
        self.snapshot_tokens
            .resize_with(state.snapshots.len(), RankedCounter::default);
        for (tokens, snapshot) in self.snapshot_tokens.iter_mut().zip(&state.snapshots) {
            tokens.load(&snapshot.tokens, self.id_ranks);
        }
        self.messages.load(&state.messages, self.id_ranks);
        self.executed_end_event_counter
            .load(&state.executed_end_event_counter, self.id_ranks);
    }

    /// Returns the ranks of the sequence flows with tokens in the snapshot with the given index of
    /// the loaded state.
    pub fn token_ranks(&self, snapshot_index: usize) -> impl Iterator<Item = usize> + '_ {
        self.snapshot_tokens[snapshot_index]
            .0
            .iter()
            .map(|&(rank, _)| rank)
    }

    /// Applies a change made by the given executor to the current state, which must be loaded.
    ///
    /// # Returns
    /// The resulting successor, which is not yet built.
    pub fn apply<'s>(
        &'s mut self,
        current_state: &'s State<'a>,
        executor: Executor<'a>,
        change: &StateChange<'a>,
    ) -> Successor<'s, 'a> {
        let id_ranks = self.id_ranks;
        let instance = match executor {
            Executor::Snapshot(index) => {
                if change.consumes_all_tokens {
                    self.successor_tokens.0.clear();
                } else {
                    self.successor_tokens
                        .0
                        .clone_from(&self.snapshot_tokens[index].0);
                }
                Instance::Existing(&current_state.snapshots[index])
            }
            Executor::NewInstance(process_id) => {
                self.successor_tokens.0.clear();
                Instance::New(process_id)
            }
        };
        for sf in change.consumes_tokens {
            self.successor_tokens.decrement(&sf.id, id_ranks);
        }
        for sf in change.produces_tokens {
            self.successor_tokens.increment(&sf.id, id_ranks);
        }

        self.successor_messages.0.clone_from(&self.messages.0);
        if let Some(mf) = change.consumes_message {
            self.successor_messages.decrement(mf, id_ranks);
        }
        for mf in change.produces_messages {
            self.successor_messages.increment(&mf.id, id_ranks);
        }

        self.successor_executed_end_event_counter
            .0
            .clone_from(&self.executed_end_event_counter.0);
        if let Some(end_event) = change.records_end_event {
            self.successor_executed_end_event_counter
                .increment(end_event, id_ranks);
        }

        Successor {
            id_ranks,
            current_state,
            instance,
            tokens: &self.successor_tokens,
            messages: &self.successor_messages,
            executed_end_event_counter: &self.successor_executed_end_event_counter,
        }
    }
}

/// A successor of the current state, whose changed parts are held by a [`SuccessorBuilder`].
#[derive(Debug)]
pub(crate) struct Successor<'s, 'a> {
    id_ranks: &'s IdRanks<'a>,
    current_state: &'s State<'a>,
    instance: Instance<'s, 'a>,
    tokens: &'s RankedCounter,
    messages: &'s RankedCounter,
    executed_end_event_counter: &'s RankedCounter,
}

impl<'a> Successor<'_, 'a> {
    /// Calculates the hash of the successor without building it.
    ///
    /// # Returns
    /// The same hash as [`State::calc_hash`] of the built successor.
    pub fn calc_hash(&self) -> u64 {
        // Hashes the same values in the same order as the derived `Hash` of `State`.
        let mut hasher = FxHasher::default();
        hash_length_prefix(self.kept_snapshot_count() + 1, &mut hasher);
        for snapshot in &self.current_state.snapshots {
            if self.instance.keeps(snapshot) {
                snapshot.hash(&mut hasher);
            }
        }
        self.instance.process_id().hash(&mut hasher);
        self.tokens.hash(self.id_ranks, &mut hasher);
        self.messages.hash(self.id_ranks, &mut hasher);
        self.executed_end_event_counter
            .hash(self.id_ranks, &mut hasher);
        hasher.finish()
    }

    pub fn build(&self) -> State<'a> {
        let mut snapshots = Vec::with_capacity(self.kept_snapshot_count() + 1);
        for snapshot in &self.current_state.snapshots {
            if self.instance.keeps(snapshot) {
                snapshots.push(snapshot.clone());
            }
        }
        snapshots.push(ProcessSnapshot {
            id: self.instance.process_id(),
            tokens: self.tokens.build(self.id_ranks),
        });
        State {
            snapshots,
            messages: self.messages.build(self.id_ranks),
            executed_end_event_counter: self.executed_end_event_counter.build(self.id_ranks),
        }
    }

    fn kept_snapshot_count(&self) -> usize {
        self.current_state
            .snapshots
            .iter()
            .filter(|snapshot| self.instance.keeps(snapshot))
            .count()
    }
}

/// Counts by the ranks of ids, sorted by rank like the maps of a [`State`] by id.
#[derive(Debug, Default)]
struct RankedCounter(Vec<(usize, u16)>);

impl RankedCounter {
    fn load(&mut self, counter: &BTreeMap<&str, u16>, id_ranks: &IdRanks) {
        self.0.clear();
        self.0.extend(
            counter
                .iter()
                .map(|(id, &count)| (id_ranks.rank(id), count)),
        );
    }

    fn increment(&mut self, id: &str, id_ranks: &IdRanks) {
        let rank = id_ranks.rank(id);
        match self
            .0
            .binary_search_by_key(&rank, |&(other_rank, _)| other_rank)
        {
            Ok(index) => self.0[index].1 += 1,
            Err(index) => self.0.insert(index, (rank, 1)),
        }
    }

    fn decrement(&mut self, id: &str, id_ranks: &IdRanks) {
        let rank = id_ranks.rank(id);
        match self
            .0
            .binary_search_by_key(&rank, |&(other_rank, _)| other_rank)
        {
            Ok(index) => {
                self.0[index].1 -= 1;
                if self.0[index].1 == 0 {
                    self.0.remove(index);
                }
            }
            Err(_) => panic!("{} should be decreased but was not present!", id),
        }
    }

    /// Hashes the counter like the equivalent map of a [`State`].
    fn hash<H: Hasher>(&self, id_ranks: &IdRanks, hasher: &mut H) {
        hash_length_prefix(self.0.len(), hasher);
        for &(rank, count) in &self.0 {
            (id_ranks.ids[rank], count).hash(hasher);
        }
    }

    fn build<'a>(&self, id_ranks: &IdRanks<'a>) -> BTreeMap<&'a str, u16> {
        self.0
            .iter()
            .map(|&(rank, count)| (id_ranks.ids[rank], count))
            .collect()
    }
}

/// Hashes the length of a collection like collections hash their length before their elements.
fn hash_length_prefix<H: Hasher>(len: usize, hasher: &mut H) {
    // A slice of `()` hashes only its length.
    vec![(); len].hash(hasher);
}
