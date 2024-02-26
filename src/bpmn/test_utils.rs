use crate::bpmn::collaboration::Collaboration;
use crate::bpmn::reader::{read_bpmn_string, UnsupportedBpmnElementsError};
use std::fs;
use std::path::Path;

pub fn read_bpmn_file(file_path: &String) -> Result<Collaboration, UnsupportedBpmnElementsError> {
    let (contents, file_name) = read_file_and_get_name(file_path);
    read_bpmn_string(&contents, file_name)
}

fn read_file_and_get_name(path: &String) -> (String, String) {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading the file {:?}. {}", path, err)
        }
    };
    (file_content, get_file_name(path))
}
fn get_file_name(path: &String) -> String {
    let path = Path::new(path);
    // Wtf is the next line. Careful file might not exist!
    path.file_name().unwrap().to_str().unwrap().parse().unwrap()
}
