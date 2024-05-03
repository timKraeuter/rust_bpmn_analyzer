mod dtos;

use crate::dtos::{CheckBPMNRequest, CheckBPMNResponse};
use axum::{http::StatusCode, routing::post, Json, Router};
use clap::Parser;
use rust_bpmn_analyzer::{read_bpmn_from_string, run, Property};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

/// CLI BPMN Analyzer written in Rust
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    /// Port to serve the web app.
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

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
    let webapp = serve_dir();

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

fn serve_dir() -> Router {
    Router::new().nest_service("/", ServeDir::new("public"))
}

async fn check_bpmn<'a>(
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
