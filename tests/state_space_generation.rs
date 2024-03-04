use bpmnanalyzer::{Config, Property};

const PATH: &str = "tests/resources/integration/";

#[test]
fn test_stable_state_space1() {
    let config = Config {
        file_path: PATH.to_string() + "p2.bpmn",
        properties: vec![
            Property::Safeness,
            Property::OptionToComplete,
            Property::ProperCompletion,
            Property::NoDeadActivities,
        ],
    };
    let result = bpmnanalyzer::run(config).unwrap();
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.transitions.len());
    assert_eq!(4, result.property_results.len());
}

#[test]
fn test_stable_state_space2() {
    let config = Config {
        file_path: PATH.to_string() + "p6.bpmn",
        properties: vec![],
    };
    let result = bpmnanalyzer::run(config).unwrap();
    assert_eq!(134, result.state_space.states.len());
    assert_eq!(134, result.state_space.transitions.len());
}

#[test]
fn test_stable_state_space_with_messages() {
    let config = Config {
        file_path: PATH.to_string() + "pools-message-flows.bpmn",
        properties: vec![],
    };
    let result = bpmnanalyzer::run(config).unwrap();
    // TODO: Should be more than 15 states!
    assert_eq!(15, result.state_space.states.len());
    assert_eq!(15, result.state_space.transitions.len());
}
