use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use rust_bpmn_analyzer_webserver::app;
use serde_json::{Value, json};
use tower::ServiceExt;

const PATH: &str = "tests/resources/";

async fn post_check_bpmn(bpmn_file: &str, properties: Value) -> (StatusCode, Value) {
    let bpmn_content = std::fs::read_to_string(format!("{PATH}{bpmn_file}"))
        .unwrap_or_else(|e| panic!("Failed to read {bpmn_file}: {e}"));

    let body = json!({
        "bpmn_file_content": bpmn_content,
        "properties_to_be_checked": properties,
    });

    let request = Request::builder()
        .method("POST")
        .uri("/check_bpmn")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::to_string(&body).unwrap(),
        ))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();

    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    (status, body)
}

fn all_properties() -> Value {
    json!([
        "Safeness",
        "OptionToComplete",
        "ProperCompletion",
        "NoDeadActivities"
    ])
}

/// Find a property result by name in the response.
fn find_property<'a>(results: &'a [Value], name: &str) -> &'a Value {
    results
        .iter()
        .find(|r| r["property"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("Property {name} not found in results"))
}

/// Assert that a state DTO has the expected structure:
/// { "snapshots": [...], "messages": {...}, "executed_end_event_counter": {...} }
fn assert_state_structure(state: &Value) {
    assert!(
        state["snapshots"].is_array(),
        "snapshots should be an array"
    );
    let snapshots = state["snapshots"].as_array().unwrap();
    for snapshot in snapshots {
        assert!(snapshot["id"].is_string(), "snapshot.id should be a string");
        assert!(
            snapshot["tokens"].is_object(),
            "snapshot.tokens should be an object"
        );
    }
    assert!(
        state["messages"].is_object(),
        "messages should be an object"
    );
    assert!(
        state["executed_end_event_counter"].is_object(),
        "executed_end_event_counter should be an object"
    );
}

#[tokio::test]
async fn test_check_bpmn_all_properties_fulfilled() {
    // p2.bpmn: Start -> parallel split -> tasks A, B -> parallel join -> End
    // All 4 properties should be fulfilled with no issues.
    let (status, body) = post_check_bpmn("p2.bpmn", all_properties()).await;

    // -- Status --
    assert_eq!(status, StatusCode::OK);

    // -- Top-level structure --
    assert!(body["property_results"].is_array());
    assert!(body["unsupported_elements"].is_array());
    assert_eq!(
        body["unsupported_elements"].as_array().unwrap().len(),
        0,
        "p2.bpmn should have no unsupported elements"
    );

    let results = body["property_results"].as_array().unwrap();
    assert_eq!(results.len(), 4, "Should have one result per property");

    // -- Safeness --
    let safeness = find_property(results, "Safeness");
    assert_eq!(safeness["fulfilled"], json!(true));
    assert_eq!(safeness["problematic_elements"], json!([]));
    assert_eq!(safeness["counter_example"], json!(null));

    // -- OptionToComplete --
    let otc = find_property(results, "OptionToComplete");
    assert_eq!(otc["fulfilled"], json!(true));
    assert_eq!(otc["problematic_elements"], json!([]));
    assert_eq!(otc["counter_example"], json!(null));

    // -- ProperCompletion --
    let pc = find_property(results, "ProperCompletion");
    assert_eq!(pc["fulfilled"], json!(true));
    assert_eq!(pc["problematic_elements"], json!([]));
    assert_eq!(pc["counter_example"], json!(null));

    // -- NoDeadActivities --
    let nda = find_property(results, "NoDeadActivities");
    assert_eq!(nda["fulfilled"], json!(true));
    assert_eq!(nda["problematic_elements"], json!([]));
    assert_eq!(nda["counter_example"], json!(null));
}

#[tokio::test]
async fn test_check_bpmn_unfulfilled_properties() {
    // p6_stuck.bpmn: 7-branch parallel split -> parallel join -> exclusive gateway
    // -> two tasks -> parallel join (stuck: XOR only sends one token, PG needs two).
    // "Dead Task" (Activity_0e5hx54) is after the stuck parallel join, so it's dead.
    // OptionToComplete and NoDeadActivities should be unfulfilled.
    let (status, body) = post_check_bpmn("p6_stuck.bpmn", all_properties()).await;

    // -- Status --
    assert_eq!(status, StatusCode::OK);

    // -- Top-level structure --
    assert!(body["property_results"].is_array());
    assert!(body["unsupported_elements"].is_array());
    assert_eq!(
        body["unsupported_elements"].as_array().unwrap().len(),
        0,
        "p6_stuck.bpmn should have no unsupported elements"
    );

    let results = body["property_results"].as_array().unwrap();
    assert_eq!(results.len(), 4, "Should have one result per property");

    // -- Safeness: fulfilled --
    let safeness = find_property(results, "Safeness");
    assert_eq!(safeness["fulfilled"], json!(true));
    assert_eq!(safeness["problematic_elements"], json!([]));
    assert_eq!(safeness["counter_example"], json!(null));

    // -- ProperCompletion: fulfilled --
    let pc = find_property(results, "ProperCompletion");
    assert_eq!(pc["fulfilled"], json!(true));
    assert_eq!(pc["problematic_elements"], json!([]));
    assert_eq!(pc["counter_example"], json!(null));

    // -- OptionToComplete: NOT fulfilled --
    let otc = find_property(results, "OptionToComplete");
    assert_eq!(otc["fulfilled"], json!(false));
    // OptionToComplete has no problematic_elements (only problematic_state_hashes)
    assert_eq!(otc["problematic_elements"], json!([]));

    // Counter example should exist and have the right structure
    let ce = &otc["counter_example"];
    assert!(ce.is_object(), "counter_example should be present");

    // Verify start_state structure
    assert_state_structure(&ce["start_state"]);

    // The start state should have exactly one snapshot (one process in p6_stuck.bpmn)
    let start_snapshots = ce["start_state"]["snapshots"].as_array().unwrap();
    assert_eq!(
        start_snapshots.len(),
        1,
        "p6_stuck.bpmn has one process, so start_state should have one snapshot"
    );
    assert_eq!(start_snapshots[0]["id"].as_str().unwrap(), "Process_1");

    // Start state should have no messages and no executed end events
    assert_eq!(ce["start_state"]["messages"], json!({}));
    assert_eq!(ce["start_state"]["executed_end_event_counter"], json!({}));

    // Transitions should be a non-empty array (there must be steps to reach the stuck state)
    let transitions = ce["transitions"].as_array().unwrap();
    assert!(
        !transitions.is_empty(),
        "Counter example should have at least one transition"
    );

    // Verify each transition has the correct structure
    for transition in transitions {
        assert!(
            transition["label"].is_string(),
            "transition.label should be a string (flow node ID)"
        );
        assert!(!transition["label"].as_str().unwrap().is_empty());
        assert_state_structure(&transition["next_state"]);
    }

    // -- NoDeadActivities: NOT fulfilled --
    let nda = find_property(results, "NoDeadActivities");
    assert_eq!(nda["fulfilled"], json!(false));

    // The dead task "Activity_0e5hx54" should be in problematic_elements
    let problematic = nda["problematic_elements"].as_array().unwrap();
    assert!(
        problematic.contains(&json!("Activity_0e5hx54")),
        "Dead Task (Activity_0e5hx54) should be a problematic element, got: {problematic:?}"
    );

    // NoDeadActivities has empty problematic_state_hashes, so counter_example should be null
    assert_eq!(nda["counter_example"], json!(null));
}
