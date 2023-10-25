use crate::configuration;
use crate::utils::{argparse, installation, notification::create_notification, setup, terminal::*, rpc};
use std::process;

static ACCEPTED_PARAMS: [(&str, &str); 7] = [
	("binary", "The binary type to launch, either Player or Studio"),
	("channel", "The deployment channel to launch"),
	("hash", "The version hash to launch"),
	("args", "The protocol arguments to launch with, usually given by a protocol"),
	("skipupdatecheck", "Skip checking for updates from clientsettings.roblox.com"),
	("debug", "Enable debug notifications"),
	("bootstrap", "Bootstrap the latest version (NOT IMPLEMENTED)"),
];

pub fn main(raw_args: &[(String, String)]) {
	if argparse::get_param_value_new(&raw_args, "help").is_some() {
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		return;
	}
	let dir_location = setup::get_applejuice_dir();
	let binary_type = argparse::get_param_value_new(&raw_args, "binary").unwrap();
	let channel = argparse::get_param_value_new(&raw_args, "channel").unwrap();
	let version_hash = argparse::get_param_value_new(&raw_args, "hash").unwrap();
	let protocol_arguments = argparse::get_param_value_new(&raw_args, "args").unwrap();

	let skip_update_check = argparse::get_param_value_new(&raw_args, "skipupdatecheck"); // Optional
	let debug_notifications = argparse::get_param_value_new(&raw_args, "debug"); // Optional
	let shall_we_bootstrap = argparse::get_param_value_new(&raw_args, "bootstrap"); // Optional

	if skip_update_check.is_none() {
		status!("Checking for updates...");
		let latest_version = installation::get_latest_version_hash(binary_type, channel);

		if &latest_version == version_hash {
			success!("You are on the latest version!");
		} else {
			warning!("You are not on the latest version! You are on {version_hash} and the latest version for {channel} is {latest_version}");
			if shall_we_bootstrap.is_some() {
				status!("Downloading and installing latest version...");
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Updating Roblox {}...", binary_type), &format!("Updating to deployment {latest_version}"));
				//args::install()
			} else {
				let formatted_install_command = format!("--install {} {}",
					if binary_type == "Player" { "client" } else { "studio" },
					if channel == "LIVE" { "" } else { channel }
				);
				create_notification("dialog-warning", "5000", "Version outdated!", &format!("You are on {} and the latest version for {} is {}\nConsider running \"{}\"", version_hash.replace("version-", ""), channel, latest_version.replace("version-", ""), formatted_install_command));
			}
		}
	}
	status!("Protocol parameter(s): {}", protocol_arguments);
	if debug_notifications.is_some() {
		create_notification("dialog-info", "15000", "Debug protocol parameters", protocol_arguments);
	}

	status!("Detecting Proton...");
	let installation_configuration = configuration::get_config(version_hash);
	let installed_deployment_location = installation_configuration["install_path"].as_str().unwrap();

	status!("Starting RPC...");
	rpc::init_rpc(binary_type.to_owned(), debug_notifications);

	status!("Launching Roblox...");
	create_notification(
		&format!("{}/assets/crudejuice.png", dir_location),
		"5000",
		&format!("Roblox {} is starting!", binary_type),
		"",
	);
	let output = process::Command::new(dbg!(format!("{}/proton", installation_configuration["preferred_proton"].as_str().unwrap())))
		.env(
			"STEAM_COMPAT_DATA_PATH",
			format!("{}/prefixdata", dir_location),
		)
		.env(
			"STEAM_COMPAT_CLIENT_INSTALL_PATH",
			format!("{}/not-steam", dir_location),
		)
		.arg("waitforexitandrun")
		.arg(format!(
			"{}/{}",
			installed_deployment_location,
			if binary_type == "Player" {
				"RobloxPlayerBeta.exe".to_string()
			} else {
				"RobloxStudioBeta.exe".to_string()
			}
		))
		.arg(protocol_arguments)
		.spawn()
		.expect("Failed to launch Roblox Player using Proton")
		.wait()
		.expect("Failed to wait on Roblox Player using Proton");

	status!("Roblox has exited with code {}", output.code().unwrap_or(0));
	create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Roblox {} has closed", binary_type), &format!("Exit code: {}", output.code().unwrap_or(0)));
}
