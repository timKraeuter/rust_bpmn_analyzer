mod bpmn;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::str;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use crate::bpmn::{BPMNCollaboration, BPMNProcess, SequenceFlow};

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
    let (contents, file_name) = read_file_and_get_name(&config.file_path);
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let mut collaboration = BPMNCollaboration {
        name: file_name,
        participants: Vec::new(),
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"process" => {
                        add_participant(&mut collaboration, e);
                    }
                    b"startEvent" => println!("start event"),
                    b"serviceTask" => println!("service task"),
                    b"parallelGateway" => println!("parallel gateway"),
                    b"exclusiveGateway" => println!("exclusive gateway"),
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"sequenceFlow" => add_sf_to_last_participant(&mut collaboration, e),
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    // println!("{:?}", collaboration);

    Ok(())
}

fn read_file_and_get_name(path: &String) -> (String, String) {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => { content }
        Err(err) => { panic!("Error reading the file {:?}. {}", path, err) }
    };
    (file_content, get_file_name(path))
}

fn get_file_name(path: &String) -> String {
    let path = Path::new(path);
    // Wtf is the next line. Careful file might not exist!
    path.file_name().unwrap().to_str().unwrap().parse().unwrap()
}

fn add_participant(collaboration: &mut BPMNCollaboration, e: BytesStart) {
    let id = get_attribute_value_or_panic(e, &String::from("id"));
    collaboration.add_participant(BPMNProcess {
        id,
        sequence_flows: Vec::new(),
        flow_nodes: Vec::new(),
    });
}

fn add_sf_to_last_participant(collaboration: &mut BPMNCollaboration, sf_bytes: BytesStart) {
    let id = get_attribute_value_or_panic(sf_bytes, &String::from("id"));
    let sf = SequenceFlow { id };

    let option = collaboration.participants.last_mut();
    match option {
        None => { panic!("Sequence flow found but no BPMN process! Malformed XML?") }
        Some(process) => {
            process.add_sf(sf);
        }
    }
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