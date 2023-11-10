use crate::model_checking::bpmn_properties::{
    BPMNProperty, BPMNPropertyResult, ModelCheckingResult,
};
use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter};

pub fn output_property_results(result: &ModelCheckingResult) {
    for property_result in result.property_results.iter() {
        if property_result.fulfilled {
            println!("{} is {}.", property_result.property, "fulfilled".green())
        } else {
            println!(
                "{} is {}. ",
                property_result.property,
                "not fulfilled".red()
            );
            print_result_unfulfilled_details(property_result);
        }
    }
}

impl Display for BPMNProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_string(self))
    }
}

fn get_string(property: &BPMNProperty) -> ColoredString {
    match property {
        BPMNProperty::Safeness => "Safeness".blue(),
        BPMNProperty::OptionToComplete => "Option to complete".blue(),
        BPMNProperty::ProperCompletion => "Proper completion".blue(),
        BPMNProperty::NoDeadActivities => "No dead activities".blue(),
    }
}

fn print_result_unfulfilled_details(property_result: &BPMNPropertyResult) {
    match property_result.property {
        BPMNProperty::Safeness => {
            println!(
                "   The sequence flow(s) {:?} can hold two or more tokens.",
                property_result.problematic_elements
            );
        }
        BPMNProperty::OptionToComplete => {
            println!();
        }
        BPMNProperty::ProperCompletion => {
            println!(
                "   The end event(s) {:?} consume two or more tokens.",
                property_result.problematic_elements
            );
        }
        BPMNProperty::NoDeadActivities => {
            println!(
                "   The activities {:?} cannot be executed.",
                property_result.problematic_elements
            );
        }
    }
}
