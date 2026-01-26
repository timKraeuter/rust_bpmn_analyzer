mod output;

use crate::output::property_info::output_property_results;
use crate::output::state_space_info::{output_por_stats, output_state_information};
use clap::Parser;
use rust_bpmn_analyzer::{
    AmpleSetConfig, ModelCheckingResult, ModelCheckingResultWithStats, Property,
};
use std::process;
use std::time::{Duration, Instant};

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

    /// Enable partial order reduction (POR) using ample sets.
    /// This can significantly reduce the state space for models with independent parallel processes.
    #[arg(long, default_value = "false")]
    pub por: bool,

    /// Show detailed POR statistics (only applies when --por is enabled).
    #[arg(long, default_value = "false")]
    pub por_stats: bool,
}

fn main() {
    let config = Config::parse();
    let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(&config.file_path);
    match collaboration {
        Ok(collaboration) => {
            let start_time = Instant::now();
            if config.por {
                let por_config = AmpleSetConfig::default();
                let result =
                    rust_bpmn_analyzer::run_with_por(&collaboration, config.properties, por_config);
                let runtime = start_time.elapsed();
                output_result_with_por(&result, runtime, config.por_stats);
            } else {
                let result = rust_bpmn_analyzer::run(&collaboration, config.properties);
                let runtime = start_time.elapsed();
                output_result(&result, runtime);
            }
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}

fn output_result(result: &ModelCheckingResult, runtime: Duration) {
    output_state_information(result, runtime);
    println!();
    output_property_results(result);
}

fn output_result_with_por(
    result: &ModelCheckingResultWithStats,
    runtime: Duration,
    show_stats: bool,
) {
    output_state_information(result.get_result(), runtime);
    if show_stats {
        println!();
        output_por_stats(result.get_stats());
    }
    println!();
    output_property_results(result.get_result());
}
