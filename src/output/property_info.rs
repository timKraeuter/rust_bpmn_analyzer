use crate::model_checking::bpmn_properties::{ModelCheckingResult, Property, PropertyResult};
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

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_string(self))
    }
}

fn get_string(property: &Property) -> ColoredString {
    match property {
        Property::Safeness => "Safeness".blue(),
        Property::OptionToComplete => "Option to complete".blue(),
        Property::ProperCompletion => "Proper completion".blue(),
        Property::NoDeadActivities => "No dead activities".blue(),
    }
}

fn print_result_unfulfilled_details(property_result: &PropertyResult) {
    match property_result.property {
        Property::Safeness => {
            println!(
                "   The sequence flow(s) {:?} can hold two or more tokens.",
                property_result.problematic_elements
            );
        }
        Property::OptionToComplete => {
            println!();
        }
        Property::ProperCompletion => {
            println!(
                "   The end event(s) {:?} consume two or more tokens.",
                property_result.problematic_elements
            );
        }
        Property::NoDeadActivities => {
            println!(
                "   The activities {:?} cannot be executed.",
                property_result.problematic_elements
            );
        }
    }
}
