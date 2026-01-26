use colored::Colorize;
use rust_bpmn_analyzer::model_checking::properties::ModelCheckingResult;
use rust_bpmn_analyzer::AmpleSetStats;
use std::time::Duration;

pub fn output_state_information(result: &ModelCheckingResult, runtime: Duration) {
    println!(
        "State space generation {} in {:?}!",
        "successful".green().bold(),
        runtime
    );
    println!(
        "States: {}, Transitions: {}",
        result.state_space.states.len(),
        result.state_space.count_transitions()
    );
    println!(
        "Terminated states: {}",
        result.state_space.terminated_state_hashes.len()
    );
}

pub fn output_por_stats(stats: &AmpleSetStats) {
    println!(
        "{}",
        "=== Partial Order Reduction Statistics ===".cyan().bold()
    );
    println!("States processed: {}", stats.states_processed);
    println!(
        "States with reduction: {} ({:.1}%)",
        stats.states_reduced,
        if stats.states_processed > 0 {
            (stats.states_reduced as f64 / stats.states_processed as f64) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "Total enabled transitions: {}",
        stats.total_enabled_transitions
    );
    println!(
        "Total explored transitions: {}",
        stats.total_explored_transitions
    );
    let reduction_pct = (1.0 - stats.reduction_ratio()) * 100.0;
    if reduction_pct > 0.0 {
        println!("Transition reduction: {:.1}%", reduction_pct);
    } else {
        println!(
            "Transition reduction: {}",
            "0% (no reduction achieved)".yellow()
        );
    }
    println!();
    println!("{}", "Ample set selection breakdown:".bold());
    println!("  Empty (no transitions):  {}", stats.reason_counts.empty);
    println!("  Single transition:       {}", stats.reason_counts.single);
    println!(
        "  Process-local (reduced): {}",
        stats.reason_counts.process_local
    );
    println!(
        "  C1 violated (expanded):  {}",
        stats.reason_counts.c1_violated
    );
    println!(
        "  C2 violated (expanded):  {}",
        stats.reason_counts.c2_violated
    );
    println!(
        "  C3 proviso (expanded):   {}",
        stats.reason_counts.c3_proviso
    );
    if stats.reason_counts.disabled > 0 {
        println!(
            "  POR disabled:            {}",
            stats.reason_counts.disabled
        );
    }
}
