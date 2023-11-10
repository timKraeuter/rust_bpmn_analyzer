use bpmnanalyzer::Config;

#[test]
fn test_stable_state_space1() {
    let config = Config {
        file_path: "tests/resources/integration/p2.bpmn".to_string(),
        properties: vec![],
        state_space_path: None,
    };
    let result = bpmnanalyzer::run(config).unwrap();
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.transitions.len());
}
#[test]
fn test_stable_state_space2() {
    let config = Config {
        file_path: "tests/resources/integration/p6.bpmn".to_string(),
        properties: vec![],
        state_space_path: None,
    };
    let result = bpmnanalyzer::run(config).unwrap();
    assert_eq!(134, result.state_space.states.len());
    assert_eq!(134, result.state_space.transitions.len());
}
