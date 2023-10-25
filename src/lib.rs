mod bpmn;

use std::error::Error;
use std::fs;
use std::str;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use crate::bpmn::{BPMNCollaboration, BPMNProcess};

pub struct Config {
    pub file_path: String,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO: Read directly from file.
    // TODO: Use serde to map to structs.
    let contents = fs::read_to_string(config.file_path)?;
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let participants = Vec::new();
    let mut collaboration = BPMNCollaboration {
        name: String::from("123"),
        participants,
    };

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"process" => {
                        let id = get_attribute_value_or_panic(e, &String::from("id"));
                        collaboration.participants.push(BPMNProcess {
                            id,
                            sequence_flows: Vec::new(),
                            flow_nodes: Vec::new(),
                        });
                    }
                    // b"startEvent" => println!("a"),
                    // b"serviceTask" => println!("a"),
                    _ => (),
                }
            }

            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    // b"process" => {                 }
                    // b"startEvent" => println!("start event end"),
                    // b"serviceTask" => println!("service task end"),
                    _ => (),
                }
            }

            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    // b"sequenceFlow" => println!("a"),
                    _ => (),
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
    }
    println!("{:?}", collaboration);

    Ok(())
}

pub fn get_attribute_value_or_panic(e: BytesStart, key: &str) -> String {
    match e.try_get_attribute(key) {
        Ok(x) => {
            match x {
                None => { panic!("Attribute value for key \"{}\" not found.", key) }
                Some(x) => {
                    match String::from_utf8(x.value.into_owned()) {
                        Ok(value) => { value }
                        Err(_) => { panic!("UTF8 Error") }
                    }
                }
            }
        }
        Err(_) => { panic!("Could not get attribute!") }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            // do something with line
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_results() {
        let query = "abc";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(expected, search(query, contents));
    }
}