use std::fs;

use crate::utils::setup;
use crate::utils::terminal::*;
use crate::utils::proton;

pub fn main() {
	status("Initialising Applejuice...");

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

	status("Finding a Proton installation...");
	let detected_installations = proton::discover_proton_directory();
	if detected_installations == "null" {
		error("Failed to find a Proton installation!");
	} else {
		status("Found the following Proton installations: ");
		for (key, _value) in detected_installations["proton_installations"].as_object().unwrap() {
			statusprogress(key);
		}

		fs::write(format!("{}/config.json", setup::get_applejuice_dir()), serde_json::to_string_pretty(&detected_installations).unwrap()).expect("Failed to write to config.json");
		success("config.json updated with Proton paths");
	}

	println!(); // "Print a newline (for aesthetics" -GitHub copilot, providing dumb crap since 2022
	success("Applejuice has been initialised!\nTo get started, run 'applejuicecli --help'\nOr to dive right in, run 'applejuicecli --install client'");
}
