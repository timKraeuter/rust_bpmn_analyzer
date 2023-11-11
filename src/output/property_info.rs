use crate::model_checking::properties::{ModelCheckingResult, Property, PropertyResult};
use crate::states::state_space::{State, StateSpace};
use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter};

pub fn output_property_results(result: &ModelCheckingResult) {
    for property_result in result.property_results.iter() {
        if property_result.fulfilled {
            println!(
                "{} is {}.",
                property_result.property,
                "fulfilled".green().bold()
            )
        } else {
            println!(
                "{} is {}. ",
                property_result.property,
                "not fulfilled".red().bold()
            );
            print_result_unfulfilled_details(property_result, &result.state_space);
        }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_property_string(self))
    }
}

fn get_property_string(property: &Property) -> ColoredString {
    match property {
        Property::Safeness => "Safeness".blue().bold(),
        Property::OptionToComplete => "Option to complete".blue().bold(),
        Property::ProperCompletion => "Proper completion".blue().bold(),
        Property::NoDeadActivities => "No dead activities".blue().bold(),
    }
}

fn print_result_unfulfilled_details(property_result: &PropertyResult, state_space: &StateSpace) {
    match property_result.property {
        Property::Safeness => {
            if property_result.problematic_elements.len() > 1 {
                println!(
                    "    The sequence flows {} can each hold two or more tokens.",
                    format!("{:?}", property_result.problematic_elements).red()
                );
            } else {
                println!(
                    "    The sequence flow {} holds two or more tokens.",
                    format!("{:?}", property_result.problematic_elements).red()
                );
            }
            print_counter_example(property_result, state_space);
        }
        Property::OptionToComplete => {
            print_counter_example(property_result, state_space);
        }
        Property::ProperCompletion => {
            if property_result.problematic_elements.len() > 1 {
                println!(
                    "    The end events {} each consume two or more tokens.",
                    format!("{:?}", property_result.problematic_elements).red()
                );
            } else {
                println!(
                    "    The end event {} consumes two or more tokens.",
                    format!("{:?}", property_result.problematic_elements).red()
                );
            }
            print_counter_example(property_result, state_space);
        }
        Property::NoDeadActivities => {
            if property_result.problematic_elements.len() > 1 {
                println!(
                    "   The activities {} cannot be executed.",
                    format!("{:?}", property_result.problematic_elements).red()
                );
            } else {
                println!(
                    "   The activity {} cannot be executed.",
                    format!("{:?}", property_result.problematic_elements).red()
                );
            }
        }
    }
}

fn print_counter_example(property_result: &PropertyResult, state_space: &StateSpace) {
    match property_result.problematic_state_hashes.first() {
        None => {}
        Some(problematic_state) => {
            match state_space.get_path_to_state(*problematic_state) {
                None => {}
                Some(path) => {
                    print!(
                        "    {}: {}",
                        "Counter example".red(),
                        state_space.get_state(&state_space.start_state_hash)
                    );
                    for (flow_node_id, state_hash) in path {
                        print!(
                            " --{}--> {}",
                            flow_node_id,
                            state_space.get_state(&state_hash)
                        );
                    }
                    println!();
                }
            };
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_state_string(self))
    }
}

fn get_state_string(state: &State) -> String {
    if state.snapshots.len() < 2 {
        match state.snapshots.first() {
            None => {}
            Some(snapshot) => {
                return format!("{:?}", snapshot.tokens);
            }
        }
    }
    todo!()
}
