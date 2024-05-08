use crate::dtos::CheckBPMNResponse;
use rust_bpmn_analyzer::{read_bpmn_from_string, Property};
use wasm_bindgen::prelude::*;

mod dtos;
#[wasm_bindgen]
pub fn check_bpmn(bpmn_content: &str) -> Result<JsValue, JsValue> {
    let collaboration = read_bpmn_from_string(bpmn_content);
    match collaboration {
        Ok(collaboration) => {
            let result = rust_bpmn_analyzer::run(
                &collaboration,
                vec![
                    Property::Safeness,
                    Property::NoDeadActivities,
                    Property::OptionToComplete,
                    Property::ProperCompletion,
                ],
            );
            Ok(serde_wasm_bindgen::to_value(&CheckBPMNResponse::from(
                result,
            ))?)
        }
        Err(error) => Ok(serde_wasm_bindgen::to_value(&CheckBPMNResponse {
            property_results: vec![],
            unsupported_elements: error.unsupported_elements,
        })?),
    }
}
