use crate::utils::setup;
use crate::utils::terminal::*;

pub fn main() {
	if setup::confirm_applejuice_data_folder_existence() {
		warning("Configuration directory already exists!");
	} else {
		println!("Constructing configuration directory...");
		setup::construct_applejuice_data_folder();
		success("Constructed configuration directory");
	}
	if setup::confirm_existence("cache") {
		warning("Cache directory already exists!");
	} else {
		setup::create_dir("cache");
		success("Created cache directory");
	}
	if setup::confirm_existence("logs") {
		warning("Logs directory already exists!");
	} else {
		setup::create_dir("logs");
		success("Created logs directory");
	}
	if setup::confirm_existence("roblox") {
		warning("Roblox directory already exists!");
	} else {
		setup::create_dir("roblox");
		success("Created Roblox directory");
	}

	println!(); // "Print a newline (for aesthetics" -GitHub copilot, providing dumb crap since 2022
	success("Applejuice has been initialised!\nTo get started, run 'applejuicecli --help'\nOr to dive right in, run 'applejuicecli --install client'");
}
