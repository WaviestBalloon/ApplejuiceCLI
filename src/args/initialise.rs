use crate::utils::setup;
use crate::utils::terminal::*;
use crate::utils::proton;
use crate::utils::configuration;

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

		configuration::update_config(detected_installations, "proton_installations");
		success("config.json updated with Proton paths");
	}

	println!(); // "Print a newline (for aesthetics" -GitHub copilot, providing dumb crap since 2022
	success("Applejuice has been initialised!\nTo get started, run 'applejuicecli --help'\nOr to dive right in, run 'applejuicecli --install client'");
}
