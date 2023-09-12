use std::fs;
use serde_json::json;

use crate::utils::setup;
use crate::utils::terminal::*;
use crate::utils::proton;
use crate::utils::configuration;


const ASSET_URLS: [&str; 3] = [
	"https://raw.githubusercontent.com/WaviestBalloon/ApplejuiceCLI/main/assets/player.png",
	"https://raw.githubusercontent.com/WaviestBalloon/ApplejuiceCLI/main/assets/studio.png",
	"https://raw.githubusercontent.com/WaviestBalloon/ApplejuiceCLI/main/assets/crudejuice.png"
];

pub fn main() {
	status!("Initialising Applejuice...");

	if setup::confirm_applejuice_data_folder_existence() {
		warning!("Configuration directory already exists!");
	} else {
		setup::construct_applejuice_data_folder();
		success!("Constructed configuration directory");
	}
	if setup::confirm_existence("cache") {
		warning!("Cache directory already exists!");
	} else {
		setup::create_dir("cache");
		success!("Created cache directory");
	}
	if setup::confirm_existence("logs") {
		warning!("Logs directory already exists!");
	} else {
		setup::create_dir("logs");
		success!("Created logs directory");
	}
	if setup::confirm_existence("roblox") {
		warning!("Roblox directory already exists!");
	} else {
		setup::create_dir("roblox");
		success!("Created Roblox directory");
	}
	if setup::confirm_existence("prefixdata") {
		warning!("Prefix directory already exists!");
	} else {
		setup::create_dir("prefixdata");
		success!("Created prefix directory");
	}

	if setup::confirm_existence("assets") {
		warning!("Assets directory already exists!");
	} else {
		setup::create_dir("assets");
		success!("Created assets directory");
		status!("Downloading assets...");
		let client = reqwest::blocking::Client::new();
		for url in ASSET_URLS.iter() {
			let filename = url.split('/').last().unwrap().to_lowercase();
			let output = client.get(url.to_string())
				.send()
				.expect("Failed to download asset")
				.bytes()
				.unwrap();

			fs::write(format!("{}/assets/{}", setup::get_applejuice_dir(), filename), output).expect("Failed to write asset");
		}
	}

	status!("Finding a Proton installation...");
	let detected_installations = proton::discover_proton_directory();
	if detected_installations == serde_json::Value::Null {
		warning!("Failed to find a Proton installation! You might not have Steam or Proton installed.");
	} else {
		status!("Found the following Proton installations: ");
		for (key, _value) in detected_installations["proton_installations"].as_object().unwrap() {
			status!("{}", key);
		}

		configuration::update_config(detected_installations, "proton_installations");
		success!("config.json updated with Proton paths");
	}

	configuration::update_config(json!({
		"config_version": "0",
		"global": {}
	}), "global");

	println!(); // "Print a newline (for aesthetics" -GitHub copilot, providing dumb crap since 2022
	success!("Applejuice has been initialised!\nTo get started, run 'applejuicecli --help'\nOr to dive right in, run 'applejuicecli --install player'");
}
