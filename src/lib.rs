mod bpmn;

use std::error::Error;
use std::str;
use crate::bpmn::{GeneralProperties, read_bpmn_file, StateSpace};

pub struct Config {
    pub file_path: String,
    pub properties: Vec<GeneralProperties>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        Ok(Config {
            file_path,
            properties: vec![GeneralProperties::Safeness],
        })
    }
}

pub fn run(config: Config) -> Result<StateSpace, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config);

    let start = collaboration.create_start_state();
    let state_space = collaboration.explore_state_space(start);

    // println!("{:?}", state_space);
    println!("Number of states: {}", state_space.states.len());

    Ok(state_space)
}