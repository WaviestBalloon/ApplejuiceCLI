use std::process;
use crate::utils::{terminal::*, setup};
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

	error("not implimented yet!");
	process::exit(1);
}
fn install_studio() {
	println!("Downloading Roblox Studio from '{}'...", DEPLOYMENT_URL_STUDIO);
	let curl_command: String = format!("curl {} -o {}{}", DEPLOYMENT_URL_STUDIO, setup::get_applejuice_dir(), "/cache/RobloxStudio.exe");
	let wine_command: String = format!("wine {}{}", setup::get_applejuice_dir(), "/cache/RobloxStudio.exe");
	// TODO: Actually download thy studio

	println!("Running: {}", curl_command);
	let command = process::Command::new("sh")
		.arg("-c")
		.arg(curl_command)
		.output()
		.expect("failed to execute process");

	//success(format!("{:?}", command.stdout));

	if command.status.success() == false {
		error(format!("Stderr was detected, download has failed.\ncurl quitted with: {}", command.status));
	} else {
		success("Downloaded Studio executable!");
	}

	println!("Launching Studio installer with Wine...\nRunning: {}", wine_command);
	let command = process::Command::new("sh")
		.arg("-c")
		.arg(wine_command)
		.output()
		.expect("failed to execute process");

	if command.status.success() == false {
		warning(format!("{}", String::from_utf8_lossy(&command.stderr)));
		error(format!("Stderr was detected, download has failed.\nWine quitted with: {}", command.status));
	} else {
		success("Studio has been installed!");
	}
}

pub fn main(parsed_args: &[String]) {
	if parsed_args.len() == 0 {
		error(format!("No command line arguments provided for install!{}", HELP_TEXT));
		process::exit(1);
	}
	let install_type: &str = &parsed_args[0];

	match install_type {
		"client" => install_client(),
		"studio" => install_studio(),
		_ => {
			error(format!("Unknown type to install '{}'{}", parsed_args[0], HELP_TEXT));
			process::exit(1);
		}
	}
}
