//! Successor states described by the changes that executing a flow node makes to a state.
//!
//! Most successors found during state space exploration were reached before. Applying a
//! [`StateChange`] to reusable buffers allows calculating the hash of a successor without
//! cloning the current state, which is then only needed for new states.

use crate::bpmn::flow_node::{MessageFlow, SequenceFlow};
use crate::states::state_space::{ProcessSnapshot, State};
use rustc_hash::FxHasher;
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

/// Reusable buffers holding the tokens, messages, and executed end events of a successor.
///
/// The buffers are sorted by key like the maps of a [`State`].
#[derive(Debug, Default)]
pub(crate) struct SuccessorBuilder<'a> {
    tokens: Vec<(&'a str, u16)>,
    messages: Vec<(&'a str, u16)>,
    executed_end_event_counter: Vec<(&'a str, u16)>,
}

impl<'a> SuccessorBuilder<'a> {
    /// Applies a change made by the given instance to the current state.
    ///
    /// # Returns
    /// The resulting successor, which is not yet built.
    pub fn apply<'s>(
        &'s mut self,
        current_state: &'s State<'a>,
        instance: Instance<'s, 'a>,
        change: &StateChange<'a>,
    ) -> Successor<'s, 'a> {
        self.tokens.clear();
        if let Instance::Existing(snapshot) = instance
            && !change.consumes_all_tokens
        {
            self.tokens
                .extend(snapshot.tokens.iter().map(|(&sf, &count)| (sf, count)));
        }
        for sf in change.consumes_tokens {
            decrement(&mut self.tokens, &sf.id);
        }
        for sf in change.produces_tokens {
            increment(&mut self.tokens, &sf.id);
        }

        self.messages.clear();
        self.messages.extend(
            current_state
                .messages
                .iter()
                .map(|(&mf, &count)| (mf, count)),
        );
        if let Some(mf) = change.consumes_message {
            decrement(&mut self.messages, mf);
        }
        for mf in change.produces_messages {
            increment(&mut self.messages, &mf.id);
        }

        self.executed_end_event_counter.clear();
        self.executed_end_event_counter.extend(
            current_state
                .executed_end_event_counter
                .iter()
                .map(|(&end_event, &count)| (end_event, count)),
        );
        if let Some(end_event) = change.records_end_event {
            increment(&mut self.executed_end_event_counter, end_event);
        }

        Successor {
            current_state,
            instance,
            tokens: &self.tokens,
            messages: &self.messages,
            executed_end_event_counter: &self.executed_end_event_counter,
        }
    }

    /// Builds the successors resulting from the given changes made by the given instance.
    pub fn build_successors(
        current_state: &State<'a>,
        instance: Instance<'_, 'a>,
        changes: &[StateChange<'a>],
    ) -> Vec<State<'a>> {
        let mut builder = SuccessorBuilder::default();
        changes
            .iter()
            .map(|change| builder.apply(current_state, instance, change).build())
            .collect()
    }
}

fn increment<'a>(counter: &mut Vec<(&'a str, u16)>, key: &'a str) {
    match counter.binary_search_by(|&(other_key, _)| other_key.cmp(key)) {
        Ok(index) => counter[index].1 += 1,
        Err(index) => counter.insert(index, (key, 1)),
    }
}

fn decrement(counter: &mut Vec<(&str, u16)>, key: &str) {
    match counter.binary_search_by(|&(other_key, _)| other_key.cmp(key)) {
        Ok(index) => {
            counter[index].1 -= 1;
            if counter[index].1 == 0 {
                counter.remove(index);
            }
        }
        Err(_) => panic!("{} should be decreased but was not present!", key),
    }
}

/// A successor of the current state, whose changed parts are held by a [`SuccessorBuilder`].
#[derive(Debug)]
pub(crate) struct Successor<'s, 'a> {
    current_state: &'s State<'a>,
    instance: Instance<'s, 'a>,
    tokens: &'s [(&'a str, u16)],
    messages: &'s [(&'a str, u16)],
    executed_end_event_counter: &'s [(&'a str, u16)],
}

impl<'a> Successor<'_, 'a> {
    /// Calculates the hash of the successor without building it.
    ///
    /// # Returns
    /// The same hash as [`State::calc_hash`] of the built successor.
    pub fn calc_hash(&self) -> u64 {
        // Hashes the same values in the same order as the derived `Hash` of `State`.
        // A sorted slice of key-value pairs hashes like the equivalent `BTreeMap`.
        let mut hasher = FxHasher::default();
        let snapshot_count = self.kept_snapshot_count() + 1;
        // A slice hashes its length before its elements. A slice of `()` hashes only its length.
        vec![(); snapshot_count].hash(&mut hasher);
        for snapshot in &self.current_state.snapshots {
            if self.keeps(snapshot) {
                snapshot.hash(&mut hasher);
            }
        }
        self.process_id().hash(&mut hasher);
        self.tokens.hash(&mut hasher);
        self.messages.hash(&mut hasher);
        self.executed_end_event_counter.hash(&mut hasher);
        hasher.finish()
    }

    pub fn build(&self) -> State<'a> {
        let mut snapshots = Vec::with_capacity(self.kept_snapshot_count() + 1);
        for snapshot in &self.current_state.snapshots {
            if self.keeps(snapshot) {
                snapshots.push(snapshot.clone());
            }
        }
        snapshots.push(ProcessSnapshot {
            id: self.process_id(),
            tokens: self.tokens.iter().copied().collect(),
        });
        State {
            snapshots,
            messages: self.messages.iter().copied().collect(),
            executed_end_event_counter: self.executed_end_event_counter.iter().copied().collect(),
        }
    }

    /// Returns whether the successor contains the given snapshot of the current state unchanged.
    fn keeps(&self, snapshot: &ProcessSnapshot) -> bool {
        match self.instance {
            Instance::Existing(executing_snapshot) => snapshot.id != executing_snapshot.id,
            Instance::New(_) => true,
        }
    }

    fn kept_snapshot_count(&self) -> usize {
        self.current_state
            .snapshots
            .iter()
            .filter(|snapshot| self.keeps(snapshot))
            .count()
    }

    fn process_id(&self) -> &'a str {
        match self.instance {
            Instance::Existing(executing_snapshot) => executing_snapshot.id,
            Instance::New(process_id) => process_id,
        }
    }
}
