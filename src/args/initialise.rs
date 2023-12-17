use std::{fs, env::var};
use serde_json::json;

use crate::utils::{setup, terminal::*, proton, configuration, steamos, argparse};

const ROOT_GITHUB_URL: &str = "https://raw.githubusercontent.com/WaviestBalloon/ApplejuiceCLI/main";
const ASSET_URLS: [&str; 3] = [
	"/assets/player.png",
	"/assets/studio.png",
	"/assets/crudejuice.png"
];
const ROBLOX_PLAYER_MIMES: &str = "x-scheme-handler/roblox-player;x-scheme-handler/roblox";
const ROBLOX_STUDIO_MIMES: &str = "x-scheme-handler/roblox-studio;x-scheme-handler/roblox-studio-auth";

static ACCEPTED_PARAMS: [(&str, &str); 1] = [
	("--sosoverride", "Overrides the check for SteamOS, adds Roblox to your Steam library")
];

pub fn main(raw_args: &[(String, String)]) {
	if argparse::get_param_value_new(&raw_args, "help").is_some() {
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		return;
	}
	status!("Initialising Applejuice...");
	let override_steamos_check = argparse::get_param_value_new(&raw_args, "sosoverride");

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
		status!("Downloading assets from GitHub...");
		let client = reqwest::blocking::Client::new();
		for url in ASSET_URLS.iter() {
			let filename = url.split('/').last().unwrap().to_lowercase();
			let output = client.get(format!("{ROOT_GITHUB_URL}{url}"))
				.send()
				.expect("Failed to download asset")
				.bytes()
				.unwrap();

			fs::write(format!("{}/assets/{}", setup::get_applejuice_dir(), filename), output).expect("Failed to write asset");
			success!("Downloaded {}!", filename);
		}
	}

	// Initialise entries for config.json
	configuration::update_config(json!({
		"config_version": "0",
		"ui": {},
		"cli": {},
		"misc": {
			"overrides": {
				"LATEST_VERSION_PLAYER_CHANNEL": null,
				"LATEST_VERSION_STUDIO_CHANNEL": null,
				"LIVE_DEPLOYMENT_CDN": null,
				"CHANNEL_DEPLOYMENT_CDN": null,
			},
			"purge_cached_deployment_after_install": false,
		}
	}), "global");
	configuration::update_config(json!({ }), "roblox_installations");

	status!("Finding a Proton installation...");
	let detected_installations = proton::discover_proton_directory();
	if detected_installations == serde_json::Value::Null {
		warning!("Failed to find a Proton installation! You might not have Steam or Proton installed.");
	} else {
		configuration::update_config(detected_installations, "proton_installations");
		success!("config.json updated with Proton paths");
	}

	status!("Creating Roblox shortcuts...");
	let location = setup::get_applejuice_dir();
	let desktop_shortcut_path = format!("{}/.local/share/applications/", var("HOME").expect("$HOME not set"));
	
	let player_shortcut_contents = format!("[Desktop Entry]
Name=Roblox Player
Comment=Launch Roblox Player with Applejuice and download latest version
Exec=env applejuicecli --launch --binary Player --bootstrap --args %u
Icon={location}/assets/player.png
Type=Application
Categories=Game
MimeType={ROBLOX_PLAYER_MIMES}");
	let studio_shortcut_contents = format!("[Desktop Entry]
Name=Roblox Studio
Comment=Launch Roblox Studio with Applejuice and download latest version
Exec=env applejuicecli --launch --binary Studio --bootstrap --args %u
Icon={location}/assets/studio.png
Type=Application
Categories=Game
MimeType={ROBLOX_STUDIO_MIMES}");

	fs::write(format!("{desktop_shortcut_path}/roblox-player.desktop"), player_shortcut_contents).expect("Failed to write desktop shortcut for Player");
	fs::write(format!("{desktop_shortcut_path}/roblox-studio.desktop"), studio_shortcut_contents).expect("Failed to write desktop shortcut for Studio");

	configuration::update_desktop_database();

	if steamos::is_running_on_steamos() || override_steamos_check.is_some() {
		status!("Detected SteamOS, automatically adding to Steam...");
		steamos::add_item_to_steam_library(format!("{desktop_shortcut_path}/roblox-player.desktop"));
		steamos::add_item_to_steam_library(format!("{desktop_shortcut_path}/roblox-studio.desktop"));
	}

	println!();
	success!("Applejuice has been initialised!\n\tTo get started, run 'applejuicecli --help'\n\tOr to dive right in, launch Roblox Player or Roblox Studio!");
}
