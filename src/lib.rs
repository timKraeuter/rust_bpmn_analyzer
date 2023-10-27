mod bpmn;

use std::error::Error;
use std::str;
use crate::bpmn::{read_bpmn_file, StateSpace};

pub struct Config {
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<StateSpace, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config);

    println!("{:?}", collaboration);

    let state_space = collaboration.explore_state_space();
    println!("{:?}", state_space);

    Ok(state_space)
}