use crate::utils::{setup, terminal::*, argparse, installation, notification::create_notification};
use crate::configuration;
use std::{process, thread};

const HELP_TEXT: &str = "\nUsage: TODO";
const DLL_OVERRIDES: [&str; 2] = [
	"ntdll.dll",
	"vulkan-1.dll"
];

pub fn main(raw_args: Vec<Vec<(String, String)>>) {
	let dir_location = setup::get_applejuice_dir();
	let binary_type = argparse::get_param_value(raw_args.clone(), "binary");
	let channel = argparse::get_param_value(raw_args.clone(), "channel");
	let version_hash = argparse::get_param_value(raw_args.clone(), "hash");
	let protocol_arguments = argparse::get_param_value(raw_args.clone(), "args");
	let skip_update_check = argparse::get_param_value(raw_args, "skipupdatecheck"); // Optional
	
	if skip_update_check.is_empty() {
		status("Checking for updates...");
		let latest_version = installation::get_latest_version_hash(&binary_type, &channel);

		if latest_version == version_hash {
			success("You are on the latest version!");
		} else {
			warning(format!("You are not on the latest version! You are on {} and the latest version for {} is {}", version_hash, channel, latest_version));
			let formatted_install_command = format!("--install {} {}", if binary_type == "Player" { "client" } else { "studio" }, if channel == "LIVE" { "" } else { &channel });
			create_notification("dialog-warning", "5000", "Version outdated!", &format!("You are on {} and the latest version for {} is {}\nConsider running \"{}\"", version_hash.replace("version-", ""), channel, latest_version.replace("version-", ""), formatted_install_command));
		}
	}
	if !protocol_arguments.is_empty() {
		create_notification("", "10000", "Protocol arguments detected!", &protocol_arguments);
	}

	status("Detecting Proton...");
	let installation_configuration = configuration::get_config(&version_hash);
	let installed_deployment_location = installation_configuration["install_path"].as_str().unwrap();
	let mut binary_location = "".to_string();

	let path_normal = format!("{}/dist/bin", installation_configuration["preferred_proton"].as_str().unwrap());
	let path_experimental = format!("{}/files/bin", installation_configuration["preferred_proton"].as_str().unwrap());

	if setup::confirm_existence_raw(&path_normal) {
		binary_location = path_normal;
	} else {
		warning(format!("Failed to find the installation at {}, attempting a different location...", path_normal));
		if setup::confirm_existence_raw(&path_experimental) {
			binary_location = path_experimental;
		} else {
			create_notification("dialog-error", "10000", "Proton installation not found!", "Exhausted all possible locations, aborting...");
			error(format!("Failed to find the installation at {}, aborting...", path_experimental));
		}
	}

	status("Configuring DDL overrides...");
	let mut configured_dll_overrides = "";

	status("Launching Roblox...");
	process::Command::new(format!("{}/wine", binary_location))
		.env("WINEPREFIX", format!("{}/prefixdata", dir_location))
		.env("STEAM_COMPAT_DATA_PATH", format!("{}/prefixdata", dir_location))
		.arg(format!("{}/{}", installed_deployment_location, if binary_type == "Player" { "RobloxPlayerBeta.exe".to_string() } else { "RobloxStudioBeta.exe".to_string() }))
		.arg(protocol_arguments)
		.spawn()
		.expect("Failed to launch Roblox Player using Proton");
}
