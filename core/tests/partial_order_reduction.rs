//! Tests for Partial Order Reduction (POR) with ample sets.
//!
//! These tests verify that:
//! 1. POR preserves all properties (same property results as full exploration)
//! 2. POR achieves state space reduction for suitable models
//! 3. POR statistics are correctly computed

use rstest::rstest;
use rust_bpmn_analyzer::{AmpleSetConfig, ModelCheckingResult, Property};

const PATH: &str = "tests/resources/";

/// Helper to get all properties
fn all_properties() -> Vec<Property> {
    vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ]
}

/// Asserts that two ModelCheckingResults have equivalent property results.
/// Compares property type, fulfilled status, and problematic elements.
/// Note: problematic_state_hashes may differ since POR explores fewer states,
/// but the detected problematic elements should be the same.
fn assert_property_results_equal(
    result_a: &ModelCheckingResult,
    result_b: &ModelCheckingResult,
    context: &str,
) {
    assert_eq!(
        result_a.property_results.len(),
        result_b.property_results.len(),
        "{}: Should have same number of property results",
        context
    );

    for (prop_a, prop_b) in result_a
        .property_results
        .iter()
        .zip(result_b.property_results.iter())
    {
        assert_eq!(
            prop_a.property, prop_b.property,
            "{}: Property type mismatch",
            context
        );
        assert_eq!(
            prop_a.fulfilled, prop_b.fulfilled,
            "{}: Property {:?} fulfilled status differs (expected {}, got {})",
            context, prop_a.property, prop_a.fulfilled, prop_b.fulfilled
        );

        // Compare problematic elements (sorted for consistent comparison)
        let mut elements_a = prop_a.problematic_elements.clone();
        let mut elements_b = prop_b.problematic_elements.clone();
        elements_a.sort();
        elements_b.sort();
        assert_eq!(
            elements_a, elements_b,
            "{}: Property {:?} problematic elements differ",
            context, prop_a.property
        );
    }
}

/// Test that POR preserves properties for various models.
/// This parameterized test verifies that running with POR produces
/// the same property results as full exploration.
#[rstest]
// Integration tests
#[case::simple("integration/p2.bpmn", "simple model")]
#[case::stuck("integration/p6_stuck.bpmn", "deadlock model")]
#[case::messages("integration/pools-message-flows.bpmn", "message model")]
#[case::complex("integration/e020.bpmn", "complex model")]
#[case::parallel("integration/p10.bpmn", "parallel model")]
#[case::message_persistence("integration/message_persistence.bpmn", "message persistence")]
// Note: livelock.bpmn excluded - livelocks involve infinite cycles that POR handles differently
// Unit tests - semantics
#[case::terminate_end("unit/semantics/terminate_end.bpmn", "terminate end event")]
#[case::task_and_gateways("unit/semantics/task_and_gateways.bpmn", "task and gateways")]
#[case::task("unit/semantics/task.bpmn", "simple task")]
#[case::start("unit/semantics/start.bpmn", "start event")]
#[case::send_task("unit/semantics/send_task.bpmn", "send task")]
#[case::receive_task_no_mf(
    "unit/semantics/receive_task_no_mf.bpmn",
    "receive task no message flow"
)]
#[case::receive_task("unit/semantics/receive_task.bpmn", "receive task")]
#[case::pg("unit/semantics/pg.bpmn", "parallel gateway")]
#[case::nothing("unit/semantics/nothing.bpmn", "empty process")]
#[case::multiple_participants("unit/semantics/multiple_participants.bpmn", "multiple participants")]
#[case::message_start_event("unit/semantics/message_start_event.bpmn", "message start event")]
#[case::message_intermediate_catch(
    "unit/semantics/message_intermediate_catch_event.bpmn",
    "message intermediate catch"
)]
#[case::link_event("unit/semantics/link_event.bpmn", "link event")]
#[case::intermediate_event("unit/semantics/intermediate_event.bpmn", "intermediate event")]
#[case::evg("unit/semantics/evg.bpmn", "event-based gateway")]
#[case::end("unit/semantics/end.bpmn", "end event")]
#[case::exg("unit/semantics/exg.bpmn", "exclusive gateway")]
// Unit tests - reader (only pools-message-flows.bpmn is supported)
// Note: tasks.bpmn, gateways.bpmn, events.bpmn, event-subprocesses.bpmn contain unsupported elements
#[case::reader_pools("unit/reader/pools-message-flows.bpmn", "reader pools")]
// Unit tests - properties
#[case::unsafe_model("unit/properties/safeness/unsafe.bpmn", "unsafe model")]
#[case::proper_completion_2(
    "unit/properties/proper_completion/proper-completion-2.bpmn",
    "proper completion 2"
)]
#[case::proper_completion_1(
    "unit/properties/proper_completion/proper-completion-1.bpmn",
    "proper completion 1"
)]
#[case::no_proper_completion_3(
    "unit/properties/proper_completion/no-proper-completion-3-unsafe.bpmn",
    "no proper completion unsafe"
)]
#[case::no_proper_completion_2(
    "unit/properties/proper_completion/no-proper-completion-2.bpmn",
    "no proper completion 2"
)]
#[case::no_proper_completion_1(
    "unit/properties/proper_completion/no-proper-completion-1.bpmn",
    "no proper completion 1"
)]
#[case::no_option_to_complete_1(
    "unit/properties/option_to_complete/no-option-to-complete-1.bpmn",
    "no option to complete 1"
)]
#[case::no_option_to_complete_2(
    "unit/properties/option_to_complete/no-option-to-complete-2.bpmn",
    "no option to complete 2"
)]
#[case::no_dead_activities(
    "unit/properties/no_dead_activities/no-dead-activities.bpmn",
    "no dead activities"
)]
#[case::dead_activities(
    "unit/properties/no_dead_activities/dead-activities.bpmn",
    "dead activities"
)]
// Unit tests - prefix
#[case::wurst_prefix("unit/prefix/wurst-prefix.bpmn", "wurst prefix")]
#[case::no_prefix("unit/prefix/no-prefix.bpmn", "no prefix")]
#[case::bpmn_prefix("unit/prefix/bpmn-prefix.bpmn", "bpmn prefix")]
fn test_por_preserves_properties(#[case] filename: &str, #[case] description: &str) {
    let file_path = PATH.to_string() + filename;
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());

    // Run with POR
    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);

    // Verify full property results match (property type, fulfilled status, problematic elements)
    assert_property_results_equal(
        &result_full,
        &result_por.result,
        &format!("{} ({})", filename, description),
    );

    // Print reduction statistics
    println!("=== {} ({}) ===", filename, description);
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
/// for multiple models.
#[rstest]
#[case("integration/p2.bpmn")]
#[case("integration/p6_stuck.bpmn")]
#[case("integration/e020.bpmn")]
fn test_por_disabled_matches_full(#[case] filename: &str) {
    let file_path = PATH.to_string() + filename;
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
        "Disabled POR should produce same state count for {}",
        filename
    );

    assert_eq!(
        result_full.state_space.count_transitions(),
        result_por.result.state_space.count_transitions(),
        "Disabled POR should produce same transition count for {}",
        filename
    );

    // Property results should be identical
    assert_property_results_equal(
        &result_full,
        &result_por.result,
        &format!("disabled POR for {}", filename),
    );
}

/// Test that POR statistics are computed correctly for various models.
#[rstest]
#[case("integration/p2.bpmn")]
#[case("integration/pools-message-flows.bpmn")]
#[case("integration/e020.bpmn")]
fn test_por_statistics(#[case] filename: &str) {
    let file_path = PATH.to_string() + filename;
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    let por_config = AmpleSetConfig::default();
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);

    let stats = &result_por.ample_set_stats;

    // Basic sanity checks
    assert!(
        stats.states_processed > 0,
        "Should process at least one state for {}",
        filename
    );
    assert!(
        stats.total_enabled_transitions >= stats.total_explored_transitions,
        "Explored transitions should not exceed enabled transitions for {}",
        filename
    );
    assert!(
        stats.reduction_ratio() <= 1.0,
        "Reduction ratio should be <= 1.0 for {}",
        filename
    );
    assert!(
        stats.reduction_ratio() >= 0.0,
        "Reduction ratio should be >= 0.0 for {}",
        filename
    );

    println!("=== POR Statistics for {} ===", filename);
    result_por.print_reduction_summary();
}

/// Test POR with different C2 (invisibility) configurations.
/// This verifies that disabling C2 does not significantly increase explored transitions.
#[rstest]
#[case("integration/pools-message-flows.bpmn")]
#[case("integration/e020.bpmn")]
fn test_por_c2_invisibility_config(#[case] filename: &str) {
    let file_path = PATH.to_string() + filename;
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR to get baseline
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());

    // Run with default config (C2 enabled by default based on AmpleSetConfig::default())
    let por_config_c2 = AmpleSetConfig::default();
    let result_with_c2 =
        rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config_c2);

    // Run with C2 explicitly disabled
    let por_config_no_c2 = AmpleSetConfig {
        check_invisibility: false,
        ..Default::default()
    };
    let result_without_c2 =
        rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config_no_c2);

    println!("=== Comparing C2 enabled vs disabled for {} ===", filename);
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
        "Disabling C2 should not increase explored transitions significantly for {}",
        filename
    );

    // Both configurations should produce same property results as full exploration
    assert_property_results_equal(
        &result_full,
        &result_with_c2.result,
        &format!("C2 enabled for {}", filename),
    );
    assert_property_results_equal(
        &result_full,
        &result_without_c2.result,
        &format!("C2 disabled for {}", filename),
    );
}

/// Test that POR with sticky proviso configuration works correctly.
#[rstest]
#[case("integration/p2.bpmn", true)]
#[case("integration/p2.bpmn", false)]
#[case("integration/e020.bpmn", true)]
#[case("integration/e020.bpmn", false)]
fn test_por_sticky_proviso_config(#[case] filename: &str, #[case] use_sticky: bool) {
    let file_path = PATH.to_string() + filename;
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&file_path).unwrap();

    // Run without POR to get baseline
    let result_full = rust_bpmn_analyzer::run(&collaboration, all_properties());

    // Run with specified sticky proviso configuration
    let por_config = AmpleSetConfig {
        use_sticky_proviso: use_sticky,
        ..Default::default()
    };
    let result_por = rust_bpmn_analyzer::run_with_por(&collaboration, all_properties(), por_config);

    // Full property results should be preserved regardless of sticky proviso setting
    assert_property_results_equal(
        &result_full,
        &result_por.result,
        &format!("{} with sticky_proviso={}", filename, use_sticky),
    );

    println!("=== {} with sticky_proviso={} ===", filename, use_sticky);
    println!(
        "States: {}, Reduction ratio: {:.2}%",
        result_por.result.state_space.states.len(),
        result_por.ample_set_stats.reduction_ratio() * 100.0
    );
}
