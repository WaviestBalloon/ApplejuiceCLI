use std::{env, process::exit};
mod utils; // Import utilities that are not necessarily commands
mod args;

use crate::utils::{terminal::*, *}; // Import modules which act as a handler for certain command parameters

#[cfg(not(target_os = "linux"))]
compile_error!("Applejuice is a Linux-only application and is not designed to be ran on any operating system other than a Linux-based system");

#[cfg(target_os = "windows")]
compile_error!("Since you are trying to compile for Windows, consider using Bloxstrap instead: https://github.com/pizzaboxer/bloxstrap/");
#[cfg(target_pointer_width = "32")]
compile_error!("Roblox no longer supports 32-bit processor architectures, please use a 64-bit processor architecture");

fn main() {
	let args: Vec<String> = env::args().collect();
	if !setup::confirm_applejuice_data_folder_existence() && args[1] != "--init" { // Initialisation warning
		warning!("Applejuice has not been initialised yet! Attempting to initialise...");
		args::initialise::main(&[("".to_string(), "".to_string())]);
		status!("Continuing with task...");
	}
	if args.len() == 1 {
		let _indentation = error!("No command line arguments provided!");
		help!("Run '{} --help' for more information.", args[0]);
		exit(1);
	}

	let command = &args[1];
	let command_clean: &str = &command.replace("--", "");
	let arguments = argparse::parse_arguments(&args);

	match command_clean {
		"help" => args::help::main(),
		"init" => args::initialise::main(&arguments),
		"install" => args::install::main(&arguments),
		"purge" => args::purge::main(arguments.into_iter().map(|item| vec![item]).collect()),
		"opendata" => args::opendata::main(),
		// TODO: fix this in above code
		"launch" => args::launch::main(&arguments),
		"sysinfo" => args::sysinfo::main(),
		_ => {
			let _indentation = error!("Unknown command parameter: {:?}", command);
			help!("Run '{} --help' for more information.", args[0]);
			exit(1);
		}
	}
}
