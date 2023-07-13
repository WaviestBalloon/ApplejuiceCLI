use std::process;
use crate::utils::{terminal::*, setup};
static LATEST_VERSION: &str = "https://setup.rbxcdn.com/version";
static DEPLOYMENT_URL_CLIENT: &str = "https://setup.rbxcdn.com/version-{}-Roblox.exe";
static DEPLOYMENT_URL_STUDIO: &str = "https://setup.rbxcdn.com/RobloxStudioLauncherBeta.exe";

const HELP_TEXT: &str = "\nUsage: --install [type]\nInstalls Roblox Client or Roblox Studio\n\nOptions:\n\tclient\tInstalls the Roblox Client\n\tstudio\tInstalls Roblox Studio";

fn get_latest_version_hash() -> String {
	let output = process::Command::new("curl")
		.arg(LATEST_VERSION)
		.output()
		.expect("Failed to execute curl command");

	if output.status.success() == false {
		error(format!("Failed to get the latest available version hash.\ncurl quitted with: {}", output.status));
	}

	let output_string = String::from_utf8_lossy(&output.stdout);
	success(format!("Received latest version hash: {}", output_string));

	return output_string.to_string();
}
fn get_package_manifest(version_hash: String) -> String {
	let output = process::Command::new("curl")
		.arg(format!("https://setup.rbxcdn.com/{}-rbxPkgManifest.txt", version_hash))
		.output()
		.expect("Failed to execute curl command");

	if output.status.success() == false {
		error(format!("Failed to get the rbxPkgManifest.txt.\ncurl quitted with: {}", output.status));
	} else if String::from_utf8_lossy(&output.stdout).contains("Error") {
		error(format!("Unexpected server response when getting the rbxPkgManifest information.\nResponse: {}", String::from_utf8_lossy(&output.stdout)));
	}

	let output_string = String::from_utf8_lossy(&output.stdout);

	return output_string.to_string();
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
	//println!("Downloading Roblox Studio from '{}'...", DEPLOYMENT_URL_STUDIO);
	println!("Installing Roblox Studio...");
	let curl_command: String = format!("curl {} -o {}{}", DEPLOYMENT_URL_STUDIO, setup::get_applejuice_dir(), "/cache/RobloxStudio.exe");
	let wine_command: String = format!("wine {}{}", setup::get_applejuice_dir(), "/cache/RobloxStudio.exe");

	warning("No version hash was provided, resolving the latest version hash...");
	let version_hash = get_latest_version_hash();
	println!("Resolving package manifest for version hash {}...", version_hash);
	let package_manifest = get_package_manifest(version_hash);
	success("Obtained rbxPkgManifest.txt successfully");

	error("not implimented yet!")
}

pub fn main(parsed_args: &[String]) { // TODO: Move this func into args mods instead of utils mods
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
