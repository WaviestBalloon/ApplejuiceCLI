use std::env;
mod utils; // Import utilities that are not necessarily commands
mod args; // Import modules which act as a handler for certain command parameters
use crate::utils::terminal::*;
use crate::utils::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	if !setup::confirm_applejuice_data_folder_existence() { // Initialisation warning
		warning("Applejuice has not been initialised yet! Attempting to initialise...");
		args::initialise::main();
		status("Continuing with task...");
	}
	if args.len() == 1 {
		error(format!("No command line arguments provided!\nRun '{} --help' for more information.", args[0]));
	}


	let command = &args[1];
	let command_clean: &str = &command.replace("--", "");
	let mut arguments: Vec<Vec<(String, String)>> = vec![]; // Collected args and their values
	let mut arg_command = String::new(); // Current command parameter being collected
	let mut arg_command_value = String::new(); // Current command parameter value being collected

	for (index, argument) in args.iter().enumerate() {
		if index == 0 { continue; }

		if argument.contains("--") {
			if arg_command.is_empty() && arg_command_value.is_empty() {
				arg_command = argument.replace("--", "");
			}
		} else {
			if arg_command_value.is_empty() {
				arg_command_value = argument.to_string(); // Construct first argument
			} else {
				arg_command_value = format!("{} {}", arg_command_value, argument); // Construct argument value and concatenate :3
			}
		}

		if index == args.len() - 1 { // Last argument so just push to vec
			arguments.push(vec![(arg_command, arg_command_value)]);
			arg_command = String::new();
			arg_command_value = String::new();
		} else {
			if args[index + 1].contains("--") { // Next argument is a command
				arguments.push(vec![(arg_command, arg_command_value)]);
				arg_command = String::new();
				arg_command_value = String::new();
			}
		}
	}

	match command_clean {
		"help" => args::help::main(),
		"init" => args::initialise::main(),
		"install" => args::install::main(arguments),
		"purge" => args::purge::main(arguments),
		"opendata" => args::opendata::main(),
		"play" => args::play::main(),
		_ => {
			error(format!("Unknown command parameter: '{}'\nRun '{} --help' for more information.", command, args[0]));
		}
	}
}
