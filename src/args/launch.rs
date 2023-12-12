use crate::utils::{argparse, installation, notification::create_notification, setup, terminal::*, rpc, configuration};
use crate::args;
use std::process;

static ACCEPTED_PARAMS: [(&str, &str); 5] = [
	("binary", "The binary type to launch, either Player or Studio"),
	//("channel", "The deployment channel to launch"),
	("hash", "The version hash to use (Automatic)"),
	("args", "The protocol arguments to launch with, usually given by a protocol"),
	("skipupdatecheck", "Skip checking for updates from clientsettings.roblox.com"),
	("bootstrap", "Automatically install the provided binary if missing or outdated")
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

	let studio_oauthing = protocol_arguments.contains("roblox-studio-auth");

	status!("Finding installation in configuration file...");
	let binary = binary_type.unwrap();
	let installations = configuration::get_config("roblox_installations");
	let configuration = configuration::get_config("global");
	let found_installation: &serde_json::Value = match installations.get(&binary) {
		Some(installation) => installation,
		None => {
			if shall_we_bootstrap.is_none() {
				error!("No installation was found for {}, you can install it using '--install' or by starting it from your application launcher", binary_type.unwrap());
				process::exit(1);
			} else {
				warning!("Unable to find a Roblox installation, bootstrapping now...");
				status!("Downloading and installing latest version...");
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "15000", "Installing Roblox...", "");

				let channel = match configuration["misc"]["overrides"]["deployment_channel"].as_str() {
					Some(channel) => channel,
					None => "LIVE",
				};
				
				args::install::main(&[("install".to_string(), binary.to_string())]);

				main(raw_args);
				return;
			}
		}
	};
	let install_configuration = found_installation["configuration"].clone();
	let install_path = found_installation["install_path"].as_str().unwrap_or_default();

	if skip_update_check.is_none() {
		status!("Checking for updates...");
		let latest_version = installation::get_latest_version_hash(binary, &found_installation["channel"].as_str().unwrap_or_default());
		let version = found_installation["version_hash"].as_str().unwrap();
		let deployment_channel = found_installation["channel"].as_str().unwrap();

		if latest_version != version {
			warning!("You are not on the latest version! You are on {version} and the latest version for {deployment_channel} is {latest_version}!");

			if shall_we_bootstrap.is_some() { // TODO: move to seperate module
				status!("Downloading and installing latest version...");
				help!("Install info: \n\tInstalling: {}\nOld version: {}\nUsing deployment channel: {}", latest_version, version, deployment_channel);
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Updating Roblox {}...", binary), &format!("Updating to deployment {latest_version}"));

				args::install::main(&[("install".to_string(), binary.to_string())]);
				
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

	println!("{:?}", found_installation);
	status!("Protocol parameter(s): {}", protocol_arguments);

	if install_configuration["enable_rpc"].as_bool().unwrap_or_default() {
		status!("Starting RPC...");
		rpc::init_rpc(binary.to_owned());
	}

	status!("Launching Roblox...");
	if studio_oauthing {
		create_notification(
			&format!("{}/assets/studio.png", dir_location),
			"5000",
			"Studio OAuth",
			"Launching Studio to authenticate...",
		);
	} else {
		create_notification(
			&format!("{}/assets/crudejuice.png", dir_location),
			"5000",
			&format!("Roblox {} is starting!", binary),
			"",
		);
	}

	let output = process::Command::new(dbg!(format!("{}/proton", found_installation["preferred_proton"].as_str().unwrap())))
		.env(
			"STEAM_COMPAT_DATA_PATH",
			format!("{}/prefixdata", dir_location),
		)
		.env(
			"STEAM_COMPAT_CLIENT_INSTALL_PATH",
			format!("{}/not-steam", dir_location),
		)
		.arg("run") // Verb `waitforexitandrun` prevents other instances from launching and queues them, not good, using `run`
		.arg(format!(
			"{}/{}",
			install_path,
			if binary == "Player" {
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

	let exitcode = output.code().unwrap_or(0);
	status!("Roblox has exited with code {}", exitcode);
	
	if studio_oauthing {
		if exitcode == 0 {
			create_notification(&format!("{}/assets/studio.png", dir_location), "5000", "Studio OAuth", "You should now be logged into Roblox Studio successfully!");
		} else {
			create_notification(&format!("{}/assets/studio.png", dir_location), "5000", "Studio OAuth",&format!("An error may have occured during the authentication process, please try again.\nExit code: {}", exitcode));
		}
	} else {
		create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Roblox {} has closed", binary), &format!("Exit code: {}", exitcode));
	}
}
