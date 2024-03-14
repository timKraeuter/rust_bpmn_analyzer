use bpmnanalyzer::Property;
use std::fs;

const PATH: &str = "tests/resources/integration/";

#[test]
fn test_stable_state_space1() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PATH.to_string() + "p2.bpmn";
    let content = fs::read_to_string(file_path)?;
    let properties = vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ];
    let result = bpmnanalyzer::run(&content, properties, false)?;
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.count_transitions());
    assert_eq!(4, result.property_results.len());
    assert_eq!(1, result.state_space.terminated_state_hashes.len());
    Ok(())
}

#[test]
fn test_stable_state_space2() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PATH.to_string() + "p6_stuck.bpmn";
    let content = fs::read_to_string(file_path)?;
    let properties = vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ];
    let result = bpmnanalyzer::run(&content, properties, false)?;
    assert_eq!(134, result.state_space.states.len());
    assert_eq!(454, result.state_space.count_transitions());
    assert_eq!(0, result.state_space.terminated_state_hashes.len());
    Ok(())
}

#[test]
fn test_stable_state_space_with_messages() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PATH.to_string() + "pools-message-flows.bpmn";
    let content = fs::read_to_string(file_path)?;
    let properties = vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ];
    let result = bpmnanalyzer::run(&content, properties, false)?;
    assert_eq!(14, result.state_space.states.len());
    assert_eq!(14, result.state_space.count_transitions());
    assert_eq!(1, result.state_space.terminated_state_hashes.len());
    Ok(())
}

#[test]
fn test_stable_state_space_with_e020() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PATH.to_string() + "e020.bpmn";
    let content = fs::read_to_string(file_path)?;
    let properties = vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ];
    let result = bpmnanalyzer::run(&content, properties, false)?;
    assert_eq!(2112, result.state_space.states.len());
    assert_eq!(3573, result.state_space.count_transitions());
    assert_eq!(30, result.state_space.terminated_state_hashes.len());
    Ok(())
}
