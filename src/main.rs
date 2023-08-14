use std::env;

mod utils; // Import utilities that are not necessarily commands
use crate::utils::terminal::*;
use crate::utils::*;
mod args; // Import modules which act as a handler for certain command parameters

fn main() {
	let args: Vec<String> = env::args().collect();
	if setup::confirm_applejuice_data_folder_existence() == false { // Initialisation warning
		warning(format!("Applejuice has not been initialised yet!\nRun '{} --init' to initialise Applejuice.\n", args[0]));
	}
	if args.len() == 1 {
		error(format!("No command line arguments provided!\nRun '{} --help' for more information.", args[0]));
	}

	let command = &args[1];
	let command_clean: &str = &args[1].replace("--", ""); // TODO: collect different params and their values
	let arguments = &args[2..];
	
	match command_clean {
		"help" => args::help::main(),
		"init" => args::initialise::main(),
		"install" => args::install::main(arguments),
		"purge" => args::purge::main(arguments),
		"opendata" => args::opendata::main(),
		_ => {
			error(format!("Unknown command parameter: '{}'\nRun '{} --help' for more information.", command, args[0]));
		}
	}
}
