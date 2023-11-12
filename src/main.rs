use axum::{http::StatusCode, routing::post, Json, Router};
use bpmnanalyzer::states::state_space::StateSpace;
use bpmnanalyzer::{run, Config, ModelCheckingResult, Property};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().route("/check_bpmn", post(check_bpmn));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::debug!("Listening on {}", addr);
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn check_bpmn(
    Json(payload): Json<CheckBPMNRequest>,
) -> (StatusCode, Json<CheckBPMNResponse>) {
    let model_checking_result = run(
        &payload.bpmn_file_content,
        payload.properties_to_be_checked,
        false,
    );
    let response = match model_checking_result {
        Ok(model_checking_result) => map_result_to_response(model_checking_result),
        Err(error) => {
            tracing::error!("{}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CheckBPMNResponse {
                    property_results: vec![],
                }),
            );
        }
    };

    (StatusCode::OK, Json(response))
}

fn map_result_to_response(model_checking_result: ModelCheckingResult) -> CheckBPMNResponse {
    let property_results = model_checking_result
        .property_results
        .into_iter()
        .map(|result| {
            let state: &StateSpace = &model_checking_result.state_space;
            MinimalPropertyResult {
                fulfilled: result.fulfilled,
                property: result.property,
                problematic_elements: result.problematic_elements,
                counter_example: CounterExample::new(result.problematic_state_hashes, state),
            }
        })
        .collect();

    CheckBPMNResponse { property_results }
}

#[derive(Deserialize)]
struct CheckBPMNRequest {
    bpmn_file_content: String,
    properties_to_be_checked: Vec<Property>,
}

#[derive(Serialize)]
struct CheckBPMNResponse {
    property_results: Vec<MinimalPropertyResult>,
}

#[derive(Serialize)]
struct MinimalPropertyResult {
    property: Property,
    fulfilled: bool,
    problematic_elements: Vec<String>,
    counter_example: Option<CounterExample>,
}
#[derive(Serialize)]
struct CounterExample {
    start_state: String,
    transitions: Vec<Transition>,
}

impl CounterExample {
    fn new(problematic_state_hashes: Vec<u64>, state_space: &StateSpace) -> Option<CounterExample> {
        match problematic_state_hashes.first() {
            None => None,
            Some(problematic_state) => match state_space.get_path_to_state(*problematic_state) {
                None => None,
                Some(path) => {
                    let transitions = path
                        .into_iter()
                        .map(|(label, state_hash)| Transition {
                            label,
                            next_state: state_space.get_state(&state_hash).to_string(),
                        })
                        .collect();
                    Some(CounterExample {
                        start_state: state_space
                            .get_state(&state_space.start_state_hash)
                            .to_string(),
                        transitions,
                    })
                }
            },
        }
    }
}

#[derive(Serialize)]
struct Transition {
    label: String, // label is the executed flow node id
    next_state: String,
}
