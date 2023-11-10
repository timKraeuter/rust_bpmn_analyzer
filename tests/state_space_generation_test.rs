use bpmnanalyzer::Config;

#[test]
fn test_stable_state_space() {
    let config = Config {
        file_path: "tests/resources/integration/p2.bpmn".to_string(),
        properties: vec![],
        state_space_path: None,
    };
    let result = bpmnanalyzer::run(config).unwrap();
    for (_, state) in result.state_space.states.iter() {
        let mut tokens = state
            .snapshots
            .get(0)
            .unwrap()
            .tokens
            .keys()
            .collect::<Vec<&String>>();
        tokens.sort();
        println!("{:?}", tokens);
    }
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.transitions.len());
}
