use rust_bpmn_analyzer::{Config, ModelCheckingResult, Property};

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
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
    assert_eq!(7, result.state_space.states.len());
    assert_eq!(7, result.state_space.count_transitions());
    assert_eq!(4, result.property_results.len());
    assert_eq!(1, result.state_space.terminated_state_hashes.len());
    assert_eq!(0, get_unfulfilled_properties(result).len());
}

#[test]
fn test_stable_state_space2() {
    let config = Config {
        file_path: PATH.to_string() + "p6_stuck.bpmn",
        properties: vec![
            Property::Safeness,
            Property::OptionToComplete,
            Property::ProperCompletion,
            Property::NoDeadActivities,
        ],
    };
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
    assert_eq!(134, result.state_space.states.len());
    assert_eq!(454, result.state_space.count_transitions());
    assert_eq!(0, result.state_space.terminated_state_hashes.len());
    assert_eq!(
        vec![Property::OptionToComplete, Property::NoDeadActivities],
        get_unfulfilled_properties(result)
    );
}

#[test]
fn test_stable_state_space_with_messages() {
    let config = Config {
        file_path: PATH.to_string() + "pools-message-flows.bpmn",
        properties: vec![
            Property::Safeness,
            Property::OptionToComplete,
            Property::ProperCompletion,
            Property::NoDeadActivities,
        ],
    };
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
    assert_eq!(14, result.state_space.states.len());
    assert_eq!(14, result.state_space.count_transitions());
    assert_eq!(1, result.state_space.terminated_state_hashes.len());
    assert_eq!(
        vec![Property::OptionToComplete],
        get_unfulfilled_properties(result)
    );
}

#[test]
fn test_stable_state_space_with_e020() {
    let config = Config {
        file_path: PATH.to_string() + "e020.bpmn",
        properties: vec![
            Property::Safeness,
            Property::OptionToComplete,
            Property::ProperCompletion,
            Property::NoDeadActivities,
        ],
    };
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
    assert_eq!(2112, result.state_space.states.len());
    assert_eq!(3573, result.state_space.count_transitions());
    assert_eq!(30, result.state_space.terminated_state_hashes.len());
    assert_eq!(
        vec![Property::OptionToComplete],
        get_unfulfilled_properties(result)
    );
}

#[test]
fn test_message_receive_prio() {
    let config = Config {
        file_path: PATH.to_string() + "message_receive_prio.bpmn",
        properties: vec![
            Property::Safeness,
            Property::OptionToComplete,
            Property::ProperCompletion,
            Property::NoDeadActivities,
        ],
    };
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path).unwrap();
    let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
    assert_eq!(11, result.state_space.states.len());
    assert_eq!(10, result.state_space.count_transitions());
    assert_eq!(2, result.state_space.terminated_state_hashes.len());
    assert_eq!(
        vec![Property::OptionToComplete],
        get_unfulfilled_properties(result)
    );
}

fn get_unfulfilled_properties(result: ModelCheckingResult) -> Vec<Property> {
    result
        .property_results
        .into_iter()
        .filter_map(|property_result| {
            if !property_result.fulfilled {
                return Some(property_result.property);
            }
            None
        })
        .collect::<Vec<_>>()
}
