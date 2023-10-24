use std::error::Error;
use std::fs;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

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

    let mut sequence_flow_amount = 0;
    let mut task_amount = 0;
    let mut start_amount = 0;
    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"startEvent" => start_amount = start_amount + 1,
                    b"serviceTask" => task_amount = task_amount + 1,
                    _ => (),
                }
            }

            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"sequenceFlow" => sequence_flow_amount = sequence_flow_amount + 1,
                    _ => (),
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
    }
    println!("Start event {}, Tasks {}, Sequence flows {}", start_amount, task_amount, sequence_flow_amount);

    Ok(())
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