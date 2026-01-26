//! Tests for Partial Order Reduction (POR) with ample sets.
//!
//! These tests verify that:
//! 1. POR preserves all properties (same property results as full exploration)
//! 2. POR achieves state space reduction for suitable models
//! 3. POR statistics are correctly computed

use rust_bpmn_analyzer::{AmpleSetConfig, ModelCheckingResult, Property};

const PATH: &str = "tests/resources/integration/";

/// Helper to get all properties
fn all_properties() -> Vec<Property> {
    vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ]
}

/// Helper to get unfulfilled properties from a result
fn get_unfulfilled_properties(result: &ModelCheckingResult) -> Vec<Property> {
    result
        .property_results
        .iter()
        .filter_map(|property_result| {
            if !property_result.fulfilled {
                return Some(property_result.property.clone());
            }
            None
        })
        .collect::<Vec<_>>()
}

/// Test that POR preserves properties for a simple model
#[test]
fn test_por_preserves_properties_simple() {
    let file_path = PATH.to_string() + "p2.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());
    let unfulfilled_full = get_unfulfilled_properties(&result_full);

    // Run with POR
    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);
    let unfulfilled_por = get_unfulfilled_properties(&result_por.result);

    // Properties should be the same
    assert_eq!(
        unfulfilled_full, unfulfilled_por,
        "POR should preserve property results"
    );

    // Print reduction statistics
    println!("=== p2.bpmn (simple model) ===");
    println!(
        "Full exploration: {} states, {} transitions",
        result_full.state_space.states.len(),
        result_full.state_space.count_transitions()
    );
    println!(
        "With POR: {} states, {} transitions",
        result_por.result.state_space.states.len(),
        result_por.result.state_space.count_transitions()
    );
    println!(
        "Reduction ratio: {:.2}%",
        result_por.ample_set_stats.reduction_ratio() * 100.0
    );
}

/// Test that POR preserves properties for a stuck model (deadlock)
#[test]
fn test_por_preserves_properties_stuck() {
    let file_path = PATH.to_string() + "p6_stuck.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());
    let unfulfilled_full = get_unfulfilled_properties(&result_full);

    // Run with POR
    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);
    let unfulfilled_por = get_unfulfilled_properties(&result_por.result);

    // Properties should be the same
    assert_eq!(
        unfulfilled_full, unfulfilled_por,
        "POR should preserve property results for stuck model"
    );

    // Print reduction statistics
    println!("=== p6_stuck.bpmn (deadlock model) ===");
    println!(
        "Full exploration: {} states, {} transitions",
        result_full.state_space.states.len(),
        result_full.state_space.count_transitions()
    );
    println!(
        "With POR: {} states, {} transitions",
        result_por.result.state_space.states.len(),
        result_por.result.state_space.count_transitions()
    );
    println!(
        "Reduction ratio: {:.2}%",
        result_por.ample_set_stats.reduction_ratio() * 100.0
    );
}

/// Test that POR preserves properties for a model with messages
#[test]
fn test_por_preserves_properties_messages() {
    let file_path = PATH.to_string() + "pools-message-flows.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());
    let unfulfilled_full = get_unfulfilled_properties(&result_full);

    // Run with POR
    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);
    let unfulfilled_por = get_unfulfilled_properties(&result_por.result);

    // Print debug info
    println!("=== pools-message-flows.bpmn (message model) ===");
    println!(
        "Full exploration: {} states, {} transitions",
        result_full.state_space.states.len(),
        result_full.state_space.count_transitions()
    );
    println!(
        "With POR: {} states, {} transitions",
        result_por.result.state_space.states.len(),
        result_por.result.state_space.count_transitions()
    );
    println!(
        "Reduction ratio: {:.2}%",
        result_por.ample_set_stats.reduction_ratio() * 100.0
    );
    println!("Full unfulfilled: {:?}", unfulfilled_full);
    println!("POR unfulfilled: {:?}", unfulfilled_por);
    result_por.print_reduction_summary();

    // Properties should be the same
    assert_eq!(
        unfulfilled_full, unfulfilled_por,
        "POR should preserve property results for message model"
    );
}

/// Test that POR preserves properties for a complex model (e020)
#[test]
fn test_por_preserves_properties_complex() {
    let file_path = PATH.to_string() + "e020.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());
    let unfulfilled_full = get_unfulfilled_properties(&result_full);

    // Run with POR
    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);
    let unfulfilled_por = get_unfulfilled_properties(&result_por.result);

    // Properties should be the same
    assert_eq!(
        unfulfilled_full, unfulfilled_por,
        "POR should preserve property results for complex model"
    );

    // Print reduction statistics
    println!("=== e020.bpmn (complex model) ===");
    println!(
        "Full exploration: {} states, {} transitions",
        result_full.state_space.states.len(),
        result_full.state_space.count_transitions()
    );
    println!(
        "With POR: {} states, {} transitions",
        result_por.result.state_space.states.len(),
        result_por.result.state_space.count_transitions()
    );
    println!(
        "Reduction ratio: {:.2}%",
        result_por.ample_set_stats.reduction_ratio() * 100.0
    );
}

/// Test that disabling POR gives the same results as the original exploration
#[test]
fn test_por_disabled_matches_full() {
    let file_path = PATH.to_string() + "p2.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR (original)
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());

    // Run with POR disabled
    let por_config = AmpleSetConfig {
        enabled: false,
        ..Default::default()
    };
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);

    // State space should be identical
    assert_eq!(
        result_full.state_space.states.len(),
        result_por.result.state_space.states.len(),
        "Disabled POR should produce same state count"
    );

    assert_eq!(
        result_full.state_space.count_transitions(),
        result_por.result.state_space.count_transitions(),
        "Disabled POR should produce same transition count"
    );
}

/// Test that POR statistics are computed correctly
#[test]
fn test_por_statistics() {
    let file_path = PATH.to_string() + "p2.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);

    let stats = &result_por.ample_set_stats;

    // Basic sanity checks
    assert!(
        stats.states_processed > 0,
        "Should process at least one state"
    );
    assert!(
        stats.total_enabled_transitions >= stats.total_explored_transitions,
        "Explored transitions should not exceed enabled transitions"
    );
    assert!(
        stats.reduction_ratio() <= 1.0,
        "Reduction ratio should be <= 1.0"
    );
    assert!(
        stats.reduction_ratio() >= 0.0,
        "Reduction ratio should be >= 0.0"
    );

    println!("=== POR Statistics for p2.bpmn ===");
    result_por.print_reduction_summary();
}

/// Test POR with C2 (invisibility) disabled for potentially better reduction
#[test]
fn test_por_without_invisibility_check() {
    let file_path = PATH.to_string() + "pools-message-flows.bpmn";
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run with default config (C2 enabled)
    let por_config_c2 = AmpleSetConfig::default();
    let result_with_c2 =
        rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config_c2);

    // Run with C2 disabled
    let por_config_no_c2 = AmpleSetConfig {
        check_invisibility: false,
        ..Default::default()
    };
    let result_without_c2 =
        rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config_no_c2);

    println!("=== Comparing C2 enabled vs disabled ===");
    println!(
        "With C2: {} states, ratio {:.2}%",
        result_with_c2.result.state_space.states.len(),
        result_with_c2.ample_set_stats.reduction_ratio() * 100.0
    );
    println!(
        "Without C2: {} states, ratio {:.2}%",
        result_without_c2.result.state_space.states.len(),
        result_without_c2.ample_set_stats.reduction_ratio() * 100.0
    );

    // Without C2, we might get more reduction (or same)
    assert!(
        result_without_c2.ample_set_stats.reduction_ratio()
            <= result_with_c2.ample_set_stats.reduction_ratio() + 0.01,
        "Disabling C2 should not increase explored transitions significantly"
    );
}
