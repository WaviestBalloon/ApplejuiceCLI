use std::env;
use std::rc::Rc;

mod utils; // Import utilities that are not necessarily commands
use crate::utils::terminal::*;
use crate::utils::*;
mod args; // Import modules which act as a handler for certain command parameters

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
	let command_clean: &str = &args[1].replace("--", ""); // TODO: collect different params and their values
	//let arguments = &args[2..];

	let mut arguments: Vec<Vec<(&str, &str)>> = vec![]; // Collected args and their values
	let mut arg_command = Rc::new(""); // Current command parameter being collected
	let mut arg_command_value: &str = ""; // Current command parameter value being collected
	let mut for_counter: usize = 0; // For loop stepper


	dbg!(arguments);

	match command_clean {
		"help" => args::help::main(),
		"init" => args::initialise::main(),
		//"install" => args::install::main(arguments),
		//"purge" => args::purge::main(arguments),
		"opendata" => args::opendata::main(),
		"play" => args::play::main(),
		_ => {
			error(format!("Unknown command parameter: '{}'\nRun '{} --help' for more information.", command, args[0]));
		}
	}
}
