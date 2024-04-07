use clap::Parser;
use rust_bpmn_analyzer::Property;
use std::process;

/// CLI BPMN Analyzer written in Rust
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    /// File path to the BPMN file.
    #[arg(short, long, required = true)]
    pub file_path: String,

    /// BPMN properties to be checked.
    #[arg(short, long, value_enum, value_delimiter = ',')]
    pub properties: Vec<Property>,
}

fn main() {
    let config = Config::parse();
    let collaboration = rust_bpmn_analyzer::read_bpmn_file(&config.file_path);
    match collaboration {
        Ok(collaboration) => rust_bpmn_analyzer::run(&collaboration, config.properties),
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
