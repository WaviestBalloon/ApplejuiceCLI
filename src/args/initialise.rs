use crate::utils::setup;
use crate::utils::terminal::*;
use crate::utils::proton;
use crate::utils::configuration;

const ASSET_URLS: [&'static str; 2] = [
	"",
	""
];

pub fn main() {
	status("Initialising Applejuice...");

	if setup::confirm_applejuice_data_folder_existence() {
		warning("Configuration directory already exists!");
	} else {
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

	if setup::confirm_existence("assets") {
		warning("Assets directory already exists!");
	} else {
		setup::create_dir("assets");
		success("Created assets directory");
		status("Downloading assets...");
		let client = reqwest::blocking::Client::new();
		
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
