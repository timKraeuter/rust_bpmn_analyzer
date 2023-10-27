mod bpmn;

use std::error::Error;
use std::str;
use crate::bpmn::{BPMNCollaboration, read_bpmn_file};

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

pub fn run(config: Config) -> Result<BPMNCollaboration, Box<dyn Error>> {
    let collaboration = read_bpmn_file(&config);

    println!("{:?}", collaboration);

    Ok(collaboration)
}