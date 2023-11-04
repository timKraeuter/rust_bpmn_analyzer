mod bpmn;

use std::error::Error;
use std::str;
use crate::bpmn::{GeneralProperty, ModelCheckingResult, read_bpmn_file};

pub struct Config {
    pub file_path: String,
    pub properties: Vec<GeneralProperty>,
}

impl Config {
    pub fn new(file_path: String) -> Config {
        Config {
            file_path,
            properties: vec![]
        }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        Ok(Config {
            file_path,
            properties: vec![GeneralProperty::Safeness],
        })
    }
}

pub fn run(config: Config) -> Result<ModelCheckingResult, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config.file_path);

    let start = collaboration.create_start_state();
    let result = collaboration.explore_state_space(start, config.properties);

    // println!("{:?}", state_space);
    println!("Number of states: {}", "tbd");
    println!("Property results: {:?}", result.property_results);

    Ok(result)
}