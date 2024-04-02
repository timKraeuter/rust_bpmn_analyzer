use axum::{http::StatusCode, routing::post, Json, Router};
use clap::Parser;
use rust_bpmn_analyzer::dtos::{CheckBPMNRequest, CheckBPMNResponse};
use rust_bpmn_analyzer::{read_bpmn_string, run, Config};
use std::net::SocketAddr;
use tokio::net::TcpListener;
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
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn using_serve_dir() -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/", ServeDir::new("public"))
}

async fn check_bpmn<'a>(
    Json(payload): Json<CheckBPMNRequest>,
) -> (StatusCode, Json<CheckBPMNResponse>) {
    let collaboration = read_bpmn_string(&payload.bpmn_file_content);
    let response = match collaboration {
        Ok(collaboration) => {
            let model_checking_result =
                run(&collaboration, payload.properties_to_be_checked, false);
            tracing::info!("{:?}", "Model checking successful");
            CheckBPMNResponse::map_result(model_checking_result)
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
