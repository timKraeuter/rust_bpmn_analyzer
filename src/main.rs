use axum::{http::StatusCode, routing::post, Json, Router};
use bpmnanalyzer::states::state_space::{State, StateSpace};
use bpmnanalyzer::{run, Config, ModelCheckingResult, Property};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "info");
    } else {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt::init();

    let checker = Router::new().route("/check_bpmn", post(check_bpmn));
    let webapp = using_serve_dir();

    let app = Router::new().nest("/", checker).nest("/", webapp);

    serve(app, config.port).await;
}
async fn serve(app: Router, port: u16) {
    // Different address for docker. Maybe also wrongly applies on normal linux?
    let addr = if cfg!(target_env = "musl") {
        SocketAddr::from(([0, 0, 0, 0], port))
    } else {
        SocketAddr::from(([127, 0, 0, 1], port))
    };
    tracing::debug!("Listening on {}", addr);
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn using_serve_dir() -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/", ServeDir::new("public"))
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
        Ok(model_checking_result) => {
            tracing::info!("{:?}", "Model checking successful");
            map_result_to_response(model_checking_result)
        }
        Err(error) => {
            tracing::warn!("{:?}", error);
            CheckBPMNResponse {
                property_results: vec![],
                unsupported_elements: error.unsupported_elements,
            }
        }
    };

    (StatusCode::OK, Json(response))
}

fn map_result_to_response(model_checking_result: ModelCheckingResult) -> CheckBPMNResponse {
    let property_results = model_checking_result
        .property_results
        .into_iter()
        .map(|mut result| {
            let state: &StateSpace = &model_checking_result.state_space;
            // Might not be needed once problematic elements and state hashes are put together in tuples.
            result.problematic_elements.sort();
            result.problematic_elements.dedup();
            MinimalPropertyResult {
                fulfilled: result.fulfilled,
                property: result.property,
                problematic_elements: result.problematic_elements,
                counter_example: CounterExample::new(result.problematic_state_hashes, state),
            }
        })
        .collect();

    CheckBPMNResponse {
        property_results,
        unsupported_elements: vec![],
    }
}

#[derive(Deserialize)]
struct CheckBPMNRequest {
    bpmn_file_content: String,
    properties_to_be_checked: Vec<Property>,
}

#[derive(Serialize)]
struct CheckBPMNResponse {
    property_results: Vec<MinimalPropertyResult>,
    unsupported_elements: Vec<String>,
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
    start_state: State,
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
                            next_state: state_space.get_state(&state_hash).clone(),
                        })
                        .collect();
                    Some(CounterExample {
                        start_state: state_space.get_state(&state_space.start_state_hash).clone(),
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
    next_state: State,
}
