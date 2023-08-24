use crate::utils::{setup, terminal::*, argparse, installation};
use crate::configuration;
use std::process;

const HELP_TEXT: &str = "\nUsage: TODO";

pub fn main(raw_args: Vec<Vec<(String, String)>>) {
	let dir_location = setup::get_applejuice_dir();
	let binary_type = argparse::get_param_value(raw_args.clone(), "binary");
	let channel = argparse::get_param_value(raw_args.clone(), "channel");
	let version_hash = argparse::get_param_value(raw_args.clone(), "hash");
	let skip_update_check = argparse::get_param_value(raw_args, "skipupdatecheck"); // Optional

	if skip_update_check.is_empty() {
		status("Checking for updates...");
		let latest_version = installation::get_latest_version_hash(&binary_type, &channel);

		if latest_version == version_hash {
			success("You are on the latest version!");
		} else {
			warning(format!("You are not on the latest version! You are on {} and the latest version for {} is {}", version_hash, channel, latest_version));
			let formatted_install_command = format!("--install {} {}", if binary_type == "Player" { "client" } else { "studio" }, if channel == "LIVE" { "" } else { &channel });
			let _ = process::Command::new("notify-send")
				.arg("--app-name=Applejuice")
				.arg("--icon=dialog-warning")
				.arg("--urgency=normal")
				.arg("--expire-time=5000")
				.arg("Version outdated!")
				.arg(format!("You are on {} and the latest version for {} is {}\nConsider running \"{}\"", version_hash.replace("version-", ""), channel, latest_version.replace("version-", ""), formatted_install_command))
				.output();
		}
	}
	
	status("Launching Roblox...");
	let proton_instances = configuration::get_config("proton_installations");
}
