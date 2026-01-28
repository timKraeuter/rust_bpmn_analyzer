//! Independence relation for BPMN transitions.
//!
//! Two transitions are independent if they can be executed in any order
//! and the result is the same. This is the foundation for partial order reduction.

use std::collections::HashSet;

/// Represents the effect of executing a transition (flow node).
///
/// A transition reads from and writes to:
/// - Token positions (sequence flows)
/// - Message queues (message flows)
/// - End event counters
#[derive(Debug, Clone)]
pub struct TransitionEffect<'a> {
    /// The flow node ID that represents this transition
    pub flow_node_id: &'a str,
    /// The process this transition belongs to
    pub process_id: &'a str,
    /// Sequence flows where tokens are consumed (read + delete)
    pub consumes_tokens: HashSet<&'a str>,
    /// Sequence flows where tokens are produced (write)
    pub produces_tokens: HashSet<&'a str>,
    /// Message flows consumed (read + delete)
    pub consumes_messages: HashSet<&'a str>,
    /// Message flows produced (write)
    pub produces_messages: HashSet<&'a str>,
    /// End events that are recorded (write)
    pub records_end_events: HashSet<&'a str>,
    /// Whether this transition is "visible" for property checking
    /// (affects safeness, proper completion, or dead activities)
    pub is_visible: bool,
}

impl<'a> TransitionEffect<'a> {
    /// Create a new empty transition effect
    pub fn new(flow_node_id: &'a str, process_id: &'a str) -> Self {
        TransitionEffect {
            flow_node_id,
            process_id,
            consumes_tokens: HashSet::new(),
            produces_tokens: HashSet::new(),
            consumes_messages: HashSet::new(),
            produces_messages: HashSet::new(),
            records_end_events: HashSet::new(),
            is_visible: false,
        }
    }

    /// Get all token positions this transition touches (reads or writes)
    pub fn all_token_positions(&self) -> HashSet<&'a str> {
        self.consumes_tokens
            .union(&self.produces_tokens)
            .copied()
            .collect()
    }

    /// Get all message flows this transition touches
    pub fn all_message_flows(&self) -> HashSet<&'a str> {
        self.consumes_messages
            .union(&self.produces_messages)
            .copied()
            .collect()
    }
}

/// Check if two transitions are independent.
///
/// Two transitions are independent if:
/// 1. They don't touch the same token positions (sequence flows)
/// 2. One doesn't produce a message that the other consumes
/// 3. They don't both record the same end event (though this is rare)
///
/// Independent transitions can be executed in any order with the same result.
pub fn are_independent(t1: &TransitionEffect, t2: &TransitionEffect) -> bool {
    // Check token position conflicts
    // Two transitions conflict if one consumes/produces a token position
    // that the other also consumes/produces
    let t1_tokens = t1.all_token_positions();
    let t2_tokens = t2.all_token_positions();

    if !t1_tokens.is_disjoint(&t2_tokens) {
        return false;
    }

    // Check message flow conflicts
    // Producer-consumer dependency: if t1 produces a message that t2 consumes (or vice versa)
    if !t1.produces_messages.is_disjoint(&t2.consumes_messages) {
        return false;
    }
    if !t2.produces_messages.is_disjoint(&t1.consumes_messages) {
        return false;
    }

    // Check if both consume the same message (race condition)
    if !t1.consumes_messages.is_disjoint(&t2.consumes_messages) {
        return false;
    }

    // End events don't typically conflict with each other (they're additive)
    // But if the same end event could be recorded by different transitions, that's a conflict
    if !t1.records_end_events.is_disjoint(&t2.records_end_events) {
        return false;
    }

    true
}

/// Check if a transition is independent of all transitions in a set
pub fn is_independent_of_all(transition: &TransitionEffect, others: &[TransitionEffect]) -> bool {
    others
        .iter()
        .all(|other| are_independent(transition, other))
}

/// Check if all transitions in one set are independent of all transitions in another set
pub fn are_sets_independent(set1: &[TransitionEffect], set2: &[TransitionEffect]) -> bool {
    for t1 in set1 {
        for t2 in set2 {
            if !are_independent(t1, t2) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_independent_transitions_different_tokens() {
        let mut t1 = TransitionEffect::new("task1", "process1");
        t1.consumes_tokens.insert("sf1");
        t1.produces_tokens.insert("sf2");

        let mut t2 = TransitionEffect::new("task2", "process2");
        t2.consumes_tokens.insert("sf3");
        t2.produces_tokens.insert("sf4");

        assert!(are_independent(&t1, &t2));
    }

    #[test]
    fn test_dependent_transitions_same_token() {
        let mut t1 = TransitionEffect::new("task1", "process1");
        t1.consumes_tokens.insert("sf1");
        t1.produces_tokens.insert("sf2");

        let mut t2 = TransitionEffect::new("task2", "process1");
        t2.consumes_tokens.insert("sf2"); // Consumes what t1 produces
        t2.produces_tokens.insert("sf3");

        assert!(!are_independent(&t1, &t2));
    }

    #[test]
    fn test_dependent_transitions_message_flow() {
        let mut t1 = TransitionEffect::new("send_task", "process1");
        t1.consumes_tokens.insert("sf1");
        t1.produces_tokens.insert("sf2");
        t1.produces_messages.insert("mf1");

        let mut t2 = TransitionEffect::new("receive_task", "process2");
        t2.consumes_tokens.insert("sf3");
        t2.produces_tokens.insert("sf4");
        t2.consumes_messages.insert("mf1"); // Consumes message from t1

        assert!(!are_independent(&t1, &t2));
    }

    #[test]
    fn test_independent_transitions_different_processes() {
        let mut t1 = TransitionEffect::new("task1", "process1");
        t1.consumes_tokens.insert("p1_sf1");
        t1.produces_tokens.insert("p1_sf2");

        let mut t2 = TransitionEffect::new("task2", "process2");
        t2.consumes_tokens.insert("p2_sf1");
        t2.produces_tokens.insert("p2_sf2");

        assert!(are_independent(&t1, &t2));
    }
}
