//! Result types for partial order reduction.
//!
//! This module contains the result wrapper that includes POR statistics
//! alongside the regular model checking results.

use crate::model_checking::por::ample_set::AmpleSetStats;
use crate::model_checking::properties::ModelCheckingResult;

/// Model checking result with partial order reduction statistics.
#[derive(Debug)]
pub struct ModelCheckingResultWithStats<'a> {
    pub result: ModelCheckingResult<'a>,
    pub ample_set_stats: AmpleSetStats,
}

impl<'a> ModelCheckingResultWithStats<'a> {
    /// Get the underlying model checking result.
    pub fn get_result(&self) -> &ModelCheckingResult<'a> {
        &self.result
    }

    /// Get the ample set statistics.
    pub fn get_stats(&self) -> &AmpleSetStats {
        &self.ample_set_stats
    }

    /// Print a summary of the reduction achieved.
    pub fn print_reduction_summary(&self) {
        let stats = &self.ample_set_stats;
        println!("=== Partial Order Reduction Statistics ===");
        println!("States processed: {}", stats.states_processed);
        println!("States with reduction: {}", stats.states_reduced);
        println!(
            "Total enabled transitions: {}",
            stats.total_enabled_transitions
        );
        println!(
            "Total explored transitions: {}",
            stats.total_explored_transitions
        );
        println!("Reduction ratio: {:.2}%", stats.reduction_ratio() * 100.0);
        println!("Ample set selection reasons:");
        println!("  Empty: {}", stats.reason_counts.empty);
        println!("  Single transition: {}", stats.reason_counts.single);
        println!("  Process-local: {}", stats.reason_counts.process_local);
        println!("  C1 violated: {}", stats.reason_counts.c1_violated);
        println!("  C2 violated: {}", stats.reason_counts.c2_violated);
        println!("  C3 proviso: {}", stats.reason_counts.c3_proviso);
        println!("  Disabled: {}", stats.reason_counts.disabled);
    }
}
