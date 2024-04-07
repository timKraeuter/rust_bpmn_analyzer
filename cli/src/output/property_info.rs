use colored::Colorize;
use rust_bpmn_analyzer::model_checking::properties::{
    ModelCheckingResult, Property, PropertyResult,
};
use rust_bpmn_analyzer::states::state_space::StateSpace;

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
