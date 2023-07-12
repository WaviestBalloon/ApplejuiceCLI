use std::env;

// Import utilities that are not necessarily commands
mod utils;
use crate::utils::terminal::{error, warning};
use crate::utils::installer;
use crate::utils::setup;
mod args; // Import modules which act as a handler for certain command parameters
use crate::args::initialise;

fn main() {
	let args: Vec<String> = env::args().collect();
	if setup::confirm_applejuice_data_folder_existence() == false {
		warning(format!("Applejuice has not been initialised yet!\nRun '{} --init' to initialise Applejuice.\n", args[0]));
	}
	if args.len() == 1 {
		error(format!("No command line arguments provided!\nRun '{} --help' for more information.", args[0]));
	}

	let command = &args[1];
	let command_clean: &str = &args[1].replace("--", "");
	let arguments = &args[2..];
	
	match command_clean {
		"help" => {
			println!("ApplejuiceCLI - A manager to get Roblox to run on Linux using Valve's Proton\n\nUsage: {} [command]\n\nCommands:\n\t--help\t\tDisplays this help message\n\t--install\tInstalls Roblox Client or Roblox Studio\n\t--init\t\tInitialises Applejuice", args[0]);
		},
		"init" => initialise::main(),
		"install" => installer::main(arguments),
		_ => {
			error(format!("Unknown command parameter: '{}'\nRun '{} --help' for more information.", command, args[0]));
		}
	}
}
