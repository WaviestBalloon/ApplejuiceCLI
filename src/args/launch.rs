use crate::utils::{argparse, installation, notification::create_notification, setup, terminal::*, rpc, configuration};
use std::process;

static ACCEPTED_PARAMS: [(&str, &str); 5] = [
	("binary", "The binary type to launch, either Player or Studio"),
	("channel", "The deployment channel to launch"),
	("hash", "The version hash to launch"),
	("args", "The protocol arguments to launch with, usually given by a protocol"),
	("skipupdatecheck", "Skip checking for updates from clientsettings.roblox.com"),
];

pub fn main(raw_args: &[(String, String)]) {
	if argparse::get_param_value_new(&raw_args, "help").is_some() {
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		return;
	}
	
	let dir_location = setup::get_applejuice_dir();
	let binary_type = argparse::get_param_value_new(&raw_args, "binary");
	let protocol_arguments = argparse::get_param_value_new(&raw_args, "args").unwrap_or_default();
	let skip_update_check = argparse::get_param_value_new(&raw_args, "skipupdatecheck"); // Optional
	let shall_we_bootstrap = argparse::get_param_value_new(&raw_args, "bootstrap"); // Optional
	if binary_type.is_none() {
		error!("Missing binary type, either Player or Studio");
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		process::exit(1);
	}

	status!("Finding installation in configuration file...");
	let binary = binary_type.unwrap();
	let installations = configuration::get_config("roblox_installations");
	let found_installation: &serde_json::Value = match installations.get(&binary) {
		Some(installation) => installation,
		None => {
			error!("No installation was found for {}, you can install it using '--install' or by starting it from your application launcher", binary_type.unwrap());
			process::exit(1);
		}
	};
	let install_configuration = found_installation["configuration"].clone();
	
	println!("{}", found_installation);
	println!("{}", install_configuration);

	println!("{}", found_installation["install_path"]);

	if skip_update_check.is_none() {
		status!("Checking for updates...");
		let latest_version = installation::get_latest_version_hash(binary, &found_installation["channel"].as_str().unwrap());
		let version = found_installation["version_hash"].as_str().unwrap();
		let deployment_channel = found_installation["channel"].as_str().unwrap();

		if latest_version != version {
			warning!("You are not on the latest version! You are on {version} and the latest version for {deployment_channel} is {latest_version}!");

			if shall_we_bootstrap.is_some() {
				status!("Downloading and installing latest version...");
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Updating Roblox {}...", binary), &format!("Updating to deployment {latest_version}"));

				let version_hash = installation::get_latest_version_hash(binary, "LIVE");
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "10000", &format!("Downloading Roblox {}...", binary), &format!("Updating to deployment {latest_version}"));
				let cache_path = installation::download_deployment(binary, version_hash.to_string(), "LIVE");
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "10000", &format!("Installing Roblox {}...", binary), &format!("Updating to deployment {latest_version}"));

				// TODO: remove old version when finished downloading

				let folder_path = format!("{}/roblox/{}/{}/{}", setup::get_applejuice_dir(), deployment_channel, binary, version_hash);
				installation::extract_deployment_zips(binary, cache_path, folder_path, false);
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Updated Roblox {}!", binary), &format!("Launching will now continue..."));
				
				main(raw_args);
				return;
			} else {
				let formatted_install_command = format!("--install {} {}",
					if binary == "Player" { "player" } else { "studio" },
					if deployment_channel == "LIVE" { "" } else { deployment_channel }
				);
				create_notification("dialog-warning", "5000", "Version outdated!", &format!("You are on {} and the latest version for {} is {}\nConsider running \"{}\"", version.replace("version-", ""), deployment_channel, latest_version.replace("version-", ""), formatted_install_command));
			}
		}
	}

	println!("{:?}", installations);
	println!("{:?}", found_installation);
	status!("Protocol parameter(s): {}", protocol_arguments);

	if install_configuration["enable_rpc"].as_bool().unwrap_or_default() {
		status!("Starting RPC...");
		rpc::init_rpc(binary.to_owned(), None);
	}

}
