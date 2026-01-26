//! Ample set computation for partial order reduction.
//!
//! An ample set is a subset of enabled transitions that satisfies conditions
//! C0-C3, ensuring that exploring only the ample set preserves the properties
//! being checked while reducing the state space.

use crate::states::independence::{are_independent, TransitionEffect};

/// Configuration for ample set computation
#[derive(Debug, Clone)]
pub struct AmpleSetConfig {
    /// Whether to use partial order reduction (can be disabled for comparison)
    pub enabled: bool,
    /// Whether to check C2 (invisibility) - can be relaxed for deadlock-only checking
    pub check_invisibility: bool,
    /// Whether to use the sticky proviso for C3 (simpler but may reduce less)
    pub use_sticky_proviso: bool,
}

impl Default for AmpleSetConfig {
    fn default() -> Self {
        AmpleSetConfig {
            enabled: true,
            // Disable C2 by default for BPMN analysis. The C2 condition (invisibility)
            // is designed for LTL property checking where the order of visible actions
            // matters. For BPMN properties (safeness, deadlock, etc.), C1 alone is
            // sufficient to preserve the properties when combined with C3.
            check_invisibility: false,
            use_sticky_proviso: true, // Simpler for BFS-based exploration
        }
    }
}

/// Result of ample set computation
#[derive(Debug)]
pub struct AmpleSetResult<'a> {
    /// The selected ample set (indices into the original enabled transitions)
    pub selected_indices: Vec<usize>,
    /// Whether this is a full expansion (ample set = all enabled)
    pub is_full_expansion: bool,
    /// Reason for the selection (for debugging)
    pub reason: AmpleSetReason,
    /// The transitions in the ample set
    pub transitions: Vec<&'a TransitionEffect<'a>>,
}

/// Reason why a particular ample set was selected
#[derive(Debug, Clone, PartialEq)]
pub enum AmpleSetReason {
    /// No enabled transitions (C0)
    Empty,
    /// Only one transition enabled, trivially ample
    SingleTransition,
    /// Selected transitions from a single process that satisfies C1
    ProcessLocal,
    /// Had to fully expand due to C1 violation (message dependencies)
    C1Violated,
    /// Had to fully expand due to C2 violation (visible transitions)
    C2Violated,
    /// Had to fully expand due to C3 proviso (cycle detection)
    C3Proviso,
    /// POR is disabled in configuration
    Disabled,
}

/// Compute the ample set for a given state.
///
/// # Arguments
/// * `enabled_transitions` - All enabled transitions in the current state
/// * `config` - Configuration for ample set computation
/// * `is_on_stack` - Whether the current state is on the DFS stack (for C3)
/// * `was_visited_before` - Whether this state was previously visited (sticky proviso)
///
/// # Returns
/// An `AmpleSetResult` containing the selected subset of transitions
pub fn compute_ample_set<'a>(
    enabled_transitions: &'a [TransitionEffect<'a>],
    config: &AmpleSetConfig,
    is_on_stack: bool,
    was_visited_before: bool,
) -> AmpleSetResult<'a> {
    // Check if POR is disabled
    if !config.enabled {
        return full_expansion(enabled_transitions, AmpleSetReason::Disabled);
    }

    // C0: Empty ample set iff no enabled transitions
    if enabled_transitions.is_empty() {
        return AmpleSetResult {
            selected_indices: vec![],
            is_full_expansion: true,
            reason: AmpleSetReason::Empty,
            transitions: vec![],
        };
    }

    // Single transition is trivially an ample set
    if enabled_transitions.len() == 1 {
        return AmpleSetResult {
            selected_indices: vec![0],
            is_full_expansion: true,
            reason: AmpleSetReason::SingleTransition,
            transitions: vec![&enabled_transitions[0]],
        };
    }

    // C3 Sticky Proviso: If this state was visited before, fully expand
    // This ensures we don't create cycles without full expansion
    if config.use_sticky_proviso && was_visited_before {
        return full_expansion(enabled_transitions, AmpleSetReason::C3Proviso);
    }

    // C3 Standard Proviso: If on DFS stack, fully expand
    if !config.use_sticky_proviso && is_on_stack {
        return full_expansion(enabled_transitions, AmpleSetReason::C3Proviso);
    }

    // IMPORTANT: For correctness, if ANY transition produces OR consumes messages,
    // we must fully expand. This is because:
    // 1. Message-producing transitions can enable transitions in other processes
    // 2. Message-consuming transitions indicate inter-process communication patterns
    //    where the order of execution matters for reachability analysis
    // A more sophisticated implementation would track potential message dependencies.
    let any_involves_messages = enabled_transitions
        .iter()
        .any(|t| !t.produces_messages.is_empty() || !t.consumes_messages.is_empty());

    if any_involves_messages {
        return full_expansion(enabled_transitions, AmpleSetReason::C1Violated);
    }

    // Try to find a single independent transition as ample set
    // This works for parallel branches within a single process or across processes
    if let Some(result) = find_singleton_ample_set(enabled_transitions, config) {
        return result;
    }

    // Fallback: Return all enabled transitions (full expansion)
    // This happens when no singleton ample set can be found
    full_expansion(enabled_transitions, AmpleSetReason::C1Violated)
}

/// Try to find a single transition that is independent of all others.
/// This is the most effective reduction - we only explore one transition
/// instead of all enabled transitions.
///
/// For this to be valid (C1), the selected transition must be independent
/// of all other enabled transitions. Since we're in a state where:
/// - The selected transition t is enabled
/// - All other transitions are also enabled
/// - t is independent of all others
///
/// Then executing t first is equivalent to any other ordering.
fn find_singleton_ample_set<'a>(
    enabled_transitions: &'a [TransitionEffect<'a>],
    config: &AmpleSetConfig,
) -> Option<AmpleSetResult<'a>> {
    // Try each transition as a potential singleton ample set
    for (idx, candidate) in enabled_transitions.iter().enumerate() {
        // C1: Check if this transition is independent of ALL other enabled transitions
        let is_independent_of_others = enabled_transitions
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .all(|(_, other)| are_independent(candidate, other));

        if !is_independent_of_others {
            continue;
        }

        // C2: If checking invisibility and this is a proper reduction,
        // we need the candidate to be invisible. However, for most BPMN properties,
        // visibility is about ensuring we don't miss property violations.
        // Since C1 guarantees the other transitions will still be enabled after
        // executing this one, and we'll eventually explore them, C2 can be relaxed.
        //
        // But for strict safety, we respect the config setting.
        if config.check_invisibility && candidate.is_visible {
            // Skip this candidate, but continue trying others
            // (there might be an invisible independent transition)
            continue;
        }

        // Found a valid singleton ample set!
        return Some(AmpleSetResult {
            selected_indices: vec![idx],
            is_full_expansion: false,
            reason: AmpleSetReason::ProcessLocal,
            transitions: vec![candidate],
        });
    }

    None
}

/// Create a full expansion result (all enabled transitions)
fn full_expansion<'a>(
    enabled_transitions: &'a [TransitionEffect<'a>],
    reason: AmpleSetReason,
) -> AmpleSetResult<'a> {
    AmpleSetResult {
        selected_indices: (0..enabled_transitions.len()).collect(),
        is_full_expansion: true,
        reason,
        transitions: enabled_transitions.iter().collect(),
    }
}

/// Statistics about ample set computation during state space exploration
#[derive(Debug, Default, Clone)]
pub struct AmpleSetStats {
    /// Number of states where ample set was computed
    pub states_processed: usize,
    /// Number of states where a proper reduction was achieved
    pub states_reduced: usize,
    /// Total transitions in full expansion
    pub total_enabled_transitions: usize,
    /// Total transitions actually explored (after reduction)
    pub total_explored_transitions: usize,
    /// Breakdown by reason
    pub reason_counts: ReasonCounts,
}

#[derive(Debug, Default, Clone)]
pub struct ReasonCounts {
    pub empty: usize,
    pub single: usize,
    pub process_local: usize,
    pub c1_violated: usize,
    pub c2_violated: usize,
    pub c3_proviso: usize,
    pub disabled: usize,
}

impl AmpleSetStats {
    pub fn record(&mut self, result: &AmpleSetResult, total_enabled: usize) {
        self.states_processed += 1;
        self.total_enabled_transitions += total_enabled;
        self.total_explored_transitions += result.selected_indices.len();

        if !result.is_full_expansion {
            self.states_reduced += 1;
        }

        match result.reason {
            AmpleSetReason::Empty => self.reason_counts.empty += 1,
            AmpleSetReason::SingleTransition => self.reason_counts.single += 1,
            AmpleSetReason::ProcessLocal => self.reason_counts.process_local += 1,
            AmpleSetReason::C1Violated => self.reason_counts.c1_violated += 1,
            AmpleSetReason::C2Violated => self.reason_counts.c2_violated += 1,
            AmpleSetReason::C3Proviso => self.reason_counts.c3_proviso += 1,
            AmpleSetReason::Disabled => self.reason_counts.disabled += 1,
        }
    }

    pub fn reduction_ratio(&self) -> f64 {
        if self.total_enabled_transitions == 0 {
            1.0
        } else {
            self.total_explored_transitions as f64 / self.total_enabled_transitions as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_transition<'a>(id: &'a str, process_id: &'a str) -> TransitionEffect<'a> {
        TransitionEffect::new(id, process_id)
    }

    #[test]
    fn test_empty_ample_set() {
        let enabled: Vec<TransitionEffect> = vec![];
        let config = AmpleSetConfig::default();
        let result = compute_ample_set(&enabled, &config, false, false);

        assert!(result.selected_indices.is_empty());
        assert_eq!(result.reason, AmpleSetReason::Empty);
    }

    #[test]
    fn test_single_transition() {
        let mut t1 = make_transition("task1", "process1");
        t1.consumes_tokens.insert("sf1");
        t1.produces_tokens.insert("sf2");

        let enabled = vec![t1];
        let config = AmpleSetConfig::default();
        let result = compute_ample_set(&enabled, &config, false, false);

        assert_eq!(result.selected_indices, vec![0]);
        assert_eq!(result.reason, AmpleSetReason::SingleTransition);
    }

    #[test]
    fn test_process_local_ample_set() {
        // Two independent transitions from different processes
        // Now we can select just one as the ample set!
        let mut t1 = make_transition("task1", "process1");
        t1.consumes_tokens.insert("p1_sf1");
        t1.produces_tokens.insert("p1_sf2");

        let mut t2 = make_transition("task2", "process2");
        t2.consumes_tokens.insert("p2_sf1");
        t2.produces_tokens.insert("p2_sf2");

        let enabled = vec![t1, t2];
        let config = AmpleSetConfig {
            enabled: true,
            check_invisibility: false, // Disable C2 for this test
            use_sticky_proviso: true,
        };
        let result = compute_ample_set(&enabled, &config, false, false);

        // Should select just one transition (they are independent)
        assert_eq!(result.selected_indices.len(), 1);
        assert_eq!(result.reason, AmpleSetReason::ProcessLocal);
        assert!(!result.is_full_expansion);
    }

    #[test]
    fn test_single_process_ample_set() {
        // Two independent transitions from the SAME process (parallel branches)
        let mut t1 = make_transition("task1", "process1");
        t1.consumes_tokens.insert("p1_sf1");
        t1.produces_tokens.insert("p1_sf2");

        let mut t2 = make_transition("task2", "process1"); // Same process!
        t2.consumes_tokens.insert("p1_sf3");
        t2.produces_tokens.insert("p1_sf4");

        let enabled = vec![t1, t2];
        let config = AmpleSetConfig {
            enabled: true,
            check_invisibility: false, // Disable C2 for this test
            use_sticky_proviso: true,
        };
        let result = compute_ample_set(&enabled, &config, false, false);

        // Should select just ONE transition since they are independent
        assert_eq!(result.selected_indices.len(), 1);
        assert_eq!(result.reason, AmpleSetReason::ProcessLocal);
        assert!(!result.is_full_expansion); // This is a proper reduction!
    }

    #[test]
    fn test_c1_violated_message_dependency() {
        // t1 sends message, t2 receives it - they are dependent
        let mut t1 = make_transition("send", "process1");
        t1.consumes_tokens.insert("p1_sf1");
        t1.produces_tokens.insert("p1_sf2");
        t1.produces_messages.insert("mf1");

        let mut t2 = make_transition("receive", "process2");
        t2.consumes_tokens.insert("p2_sf1");
        t2.produces_tokens.insert("p2_sf2");
        t2.consumes_messages.insert("mf1");

        let enabled = vec![t1, t2];
        let config = AmpleSetConfig {
            enabled: true,
            check_invisibility: false,
            use_sticky_proviso: true,
        };
        let result = compute_ample_set(&enabled, &config, false, false);

        // Should fully expand due to message dependency
        assert_eq!(result.selected_indices.len(), 2);
        assert_eq!(result.reason, AmpleSetReason::C1Violated);
        assert!(result.is_full_expansion);
    }

    #[test]
    fn test_sticky_proviso() {
        let mut t1 = make_transition("task1", "process1");
        t1.consumes_tokens.insert("p1_sf1");

        let mut t2 = make_transition("task2", "process2");
        t2.consumes_tokens.insert("p2_sf1");

        let enabled = vec![t1, t2];
        let config = AmpleSetConfig::default();

        // With was_visited_before = true, should fully expand
        let result = compute_ample_set(&enabled, &config, false, true);
        assert_eq!(result.reason, AmpleSetReason::C3Proviso);
        assert!(result.is_full_expansion);
    }

    #[test]
    fn test_por_disabled() {
        let t1 = make_transition("task1", "process1");
        let t2 = make_transition("task2", "process2");

        let enabled = vec![t1, t2];
        let config = AmpleSetConfig {
            enabled: false,
            ..Default::default()
        };
        let result = compute_ample_set(&enabled, &config, false, false);

        assert_eq!(result.reason, AmpleSetReason::Disabled);
        assert!(result.is_full_expansion);
    }
}
