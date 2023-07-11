use std::process;
use std::fs;
static LATEST_VERSION: &str = "https://setup.rbxcdn.com/version";
static DEPLOYMENT_URL_CLIENT: &str = "https://setup.rbxcdn.com/version-{}-Roblox.exe";
static DEPLOYMENT_URL_STUDIO: &str = "https://setup.rbxcdn.com/RobloxStudioLauncherBeta.exe";

const HELP_TEXT: &str = "\nUsage: --install [type]\nInstalls Roblox Client or Roblox Studio\n\nOptions:\n\tclient\tInstalls the Roblox Client\n\tstudio\tInstalls Roblox Studio";

#[warn(dead_code)]
fn get_latest_version_hash() {
	// TODO
	// let mut response = reqwest::get(LATEST_VERSION).unwrap();
	// assert!(response.status().is_success());
	// let body = response.text().unwrap();
	// println!("body = {:?}", body);
	// body
}
fn install_client() {
	// let latest_version_hash = get_latest_version_hash();
	println!("Sending request to '{}' to obtain the latest version hash...", LATEST_VERSION);
	// TODO: Actually send the request
	println!("Downloading Roblox from '{}'...", DEPLOYMENT_URL_CLIENT);
	// TODO: Actually download the client
	println!("Installing Roblox Client...");
	// TODO: Actually install the client

	println!("not implimented yet!");
	process::exit(1);
}
fn install_studio() {
	println!("Downloading Roblox Studio from '{}'...", DEPLOYMENT_URL_STUDIO);
	// TODO: Actually download thy studio
}

pub fn main(parsed_args: &[String]) {
	if parsed_args.len() == 0 {
		println!("No command line arguments provided for install!{}", HELP_TEXT);
		process::exit(1);
	}
	let install_type: &str = &parsed_args[0];

	match install_type {
		"client" => install_client(),
		"studio" => install_studio(),
		_ => {
			println!("Unknown type to install '{}'{}", parsed_args[0], HELP_TEXT);
			process::exit(1);
		}
	}
}
