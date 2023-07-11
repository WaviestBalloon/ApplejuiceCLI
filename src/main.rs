use std::env;
use std::process;

mod utils;
use crate::utils::installer;
use crate::utils::setup;

fn main() {
	setup::confirm_applejuice_data_folder_existence();
	let args: Vec<String> = env::args().collect();
	
	if args.len() == 1 {
		println!("No command line arguments provided!\nRun '{} --help' for more information.", args[0]);
		process::exit(1);
	}

	let command = &args[1];
	let command_clean = &args[1].replace("--", "");
	let arguments = &args[2..];
	
	if command_clean == "install" {
		installer::main(arguments);
	} else {
		println!("Unknown command '{}'\nRun '{} --help' for more information.", command, args[0]);
		process::exit(1);
	}
}
