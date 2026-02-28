use clap::Parser;
use rust_bpmn_analyzer_webserver::app;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

const LOG_LEVEL: &str = if cfg!(debug_assertions) {
    "info"
} else {
    "warn"
};

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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(LOG_LEVEL))
        .init();

    serve(app(), config.port).await;
}

async fn serve(app: axum::Router, port: u16) {
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
