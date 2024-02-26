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
    let result = bpmnanalyzer::run(&content, properties, false).unwrap();
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.transitions.len());
    assert_eq!(4, result.property_results.len());
    Ok(())
}

#[test]
fn test_stable_state_space2() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PATH.to_string() + "p6.bpmn";
    let content = fs::read_to_string(file_path)?;
    let properties = vec![
        Property::Safeness,
        Property::OptionToComplete,
        Property::ProperCompletion,
        Property::NoDeadActivities,
    ];
    let result = bpmnanalyzer::run(&content, properties, false).unwrap();
    assert_eq!(134, result.state_space.states.len());
    assert_eq!(134, result.state_space.transitions.len());
    Ok(())
}
