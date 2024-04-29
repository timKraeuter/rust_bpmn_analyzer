use rust_bpmn_analyzer::{read_bpmn_from_string, Property};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn check_bpmn(bpmn_content: &str) {
    let collaboration = read_bpmn_from_string(bpmn_content);
    match collaboration {
        Ok(collaboration) => {
            let _result = rust_bpmn_analyzer::run(
                &collaboration,
                vec![
                    Property::Safeness,
                    Property::NoDeadActivities,
                    Property::OptionToComplete,
                    Property::ProperCompletion,
                ],
            );
            alert(&format!("BPMN check done!"));
        }
        Err(err) => {
            alert(&format!("BPMN was not ok: {}!", err));
        }
    }
    // todo!("TBD make enum wrapping error and the current result");
}
