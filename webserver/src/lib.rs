mod dtos;

use crate::dtos::{CheckBPMNRequest, CheckBPMNResponse};
use axum::{Json, Router, http::StatusCode, routing::post};
use rust_bpmn_analyzer::{Property, read_bpmn_from_string, run};
use tower_http::services::ServeDir;

pub fn app() -> Router {
    let checker = Router::new().route("/check_bpmn", post(check_bpmn));
    let webapp = serve_dir();
    Router::new().merge(checker).merge(webapp)
}

fn serve_dir() -> Router {
    Router::new().fallback_service(ServeDir::new("public"))
}

async fn check_bpmn(
    Json(payload): Json<CheckBPMNRequest>,
) -> (StatusCode, Json<CheckBPMNResponse>) {
    let collaboration = read_bpmn_from_string(&payload.bpmn_file_content);
    let response = match collaboration {
        Ok(collaboration) => {
            let properties = payload
                .properties_to_be_checked
                .into_iter()
                .map(Property::from)
                .collect();
            let model_checking_result = run(&collaboration, properties);
            tracing::info!("{:?}", "Model checking successful");
            CheckBPMNResponse::from(model_checking_result)
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
