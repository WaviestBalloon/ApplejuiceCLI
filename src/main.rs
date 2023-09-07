use std::{env, process::exit};
mod utils; // Import utilities that are not necessarily commands
mod args; use utils::argparse::parse_arguments;

use crate::utils::{terminal::*, *}; // Import modules which act as a handler for certain command parameters

#[cfg(not(target_os = "linux"))]
compile_error!("Applejuice is a Linux-only application and is not designed to be ran on any operating system other than a Linux-based system.");

#[cfg(target_os = "windows")]
compile_error!("Since you are compiling for Windows, consider using Bloxstrap: https://github.com/pizzaboxer/bloxstrap/");

fn main() {
	let args: Vec<String> = env::args().collect();
	if !setup::confirm_applejuice_data_folder_existence() && args[1] != "init" { // Initialisation warning
		warning!("Applejuice has not been initialised yet! Attempting to initialise...");
		args::initialise::main();
		status!("Continuing with task...");
	}
	if args.len() == 1 {
		let _indentation = error!("No command line arguments provided!");
		help!("Run '{} --help' for more information.", args[0]);
		exit(1);
	}

	let command = &args[1];
	let command_clean: &str = &command.replace("--", "");
	let arguments = parse_arguments(&args);

	match command_clean {
		"help" => args::help::main(),
		"init" => args::initialise::main(),
		"install" => args::install::main(&arguments),
		"purge" => args::purge::main(arguments.into_iter().map(|item| vec![item]).collect()),
		"opendata" => args::opendata::main(),
		"play" => args::play::main(),
		// TODO: fix this in above code
		"launch" => args::launch::main(&arguments),
		_ => {
			let _indentation = error!("Unknown command parameter: '{:?}'", command);
			help!("Run '{} --help' for more information.", args[0]);
			exit(1);
		}
	}
}
