use crate::utils::{argparse, installation, notification::create_notification, setup, terminal::*, rpc, configuration, steamos};
use crate::args;
use std::{process, path::Path};
use inotify::{Inotify, WatchMask};

static ACCEPTED_PARAMS: [(&str, &str); 6] = [
	("--binary", "The binary type to launch, either Player or Studio"),
	("--hash", "The version hash to use (Automatic)"),
	("--args", "The protocol arguments to launch with, usually given by a protocol"),
	("--skipupdatecheck", "Skip checking for updates from clientsettings.roblox.com"),
	("--bootstrap", "Automatically install the provided binary if missing or outdated"),
	("--sosoverride", "Overrides the check for SteamOS, does some pre/post-launch configuration to make Roblox work better on SteamOS")
];

pub fn resolve_active_logfile(expected_log_directory: String) -> Option<String> {
	let mut inotify = Inotify::init().expect("Failed to initialise inotify");
	let mut buffer = [0; 1024];

	let _ = inotify.watches().add(expected_log_directory.clone(), WatchMask::CREATE);
	status!("Waiting for log file...");

	let mut event = inotify.read_events_blocking(&mut buffer).expect("Failed to read_events_blocking");
	let file = loop {
		match event.next() {
			Some(received_event) => {
				let filename = received_event.name.unwrap().to_string_lossy();
				if filename.contains("last.log") {
					inotify.watches().remove(received_event.wd).expect("Error removing watch");
					break Some(filename);
				}
			},
			None => break None,
		}
	};

	if let Some(file) = file {
		let log_path = format!("{expected_log_directory}{file}");
		success!("Found log file at {}", log_path);

		return Some(log_path);
	} else {
		error!("Failed to find log file");
		return None;
	}
}

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
	let override_steamos_check = argparse::get_param_value_new(&raw_args, "sosoverride"); // Optional
	if binary_type.is_none() {
		error!("Missing binary type, either Player or Studio");
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		process::exit(1);
	}

	status!("Finding installation in configuration file...");
	let binary = binary_type.unwrap();
	let installations = configuration::get_config("roblox_installations");
	let configuration = configuration::get_config("global");
	let found_installation: &serde_json::Value = match installations.get(&binary) {
		Some(installation) => installation,
		None => {
			if shall_we_bootstrap.is_none() {
				error!("No installation was found for {}, you can install it by appending '--bootstrap' onto this command or by starting it from your application launcher", binary_type.unwrap());
				process::exit(1);
			}

			warning!("Unable to find a Roblox installation, bootstrapping now...");
			status!("Downloading and installing latest version...");
			create_notification(&format!("{}/assets/crudejuice.png", dir_location), 15000, &format!("Installing Roblox {}...", binary), "");

			// TODO: Remove this, as Roblox has now locked all non-prod deployment channels :c
			let _channel = match configuration["misc"]["overrides"]["deployment_channel"].as_str() {
				Some(channel) => channel,
				None => "LIVE",
			};
			
			args::install::main(&[("install".to_string(), binary.to_string())]);

			main(raw_args);
			return;
		}
	};

	let install_configuration = found_installation["configuration"].clone();
	let install_path = found_installation["install_path"].as_str().unwrap_or_default();
	let studio_oauthing = protocol_arguments.contains("roblox-studio-auth");

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
				create_notification(&format!("{}/assets/crudejuice.png", dir_location), 5000, &format!("Updating Roblox {}...", binary), &format!("Updating to deployment {latest_version}"));

				args::install::main(&[("install".to_string(), binary.to_string())]);
				
				main(raw_args);
				return;
			} else {
				let formatted_install_command = format!("--install {} {}",
					if binary == "Player" { "player" } else { "studio" },
					if deployment_channel == "LIVE" { "" } else { deployment_channel }
				);
				create_notification("dialog-warning", 5000, "Version outdated!", &format!("You are on {} and the latest version for {} is {}\nConsider running \"{}\"", version.replace("version-", ""), deployment_channel, latest_version.replace("version-", ""), formatted_install_command));
			}
		}
	}

	status!("Protocol parameter(s): {}", protocol_arguments);

	if install_configuration["enable_rpc"].as_bool().unwrap_or_default() {
		status!("Starting RPC...");
		rpc::init_rpc(binary.to_owned(), None);
	}

	let old_fullscreen_value = steamos::get_fullscreen_value_from_rbxxml().unwrap_or_default();
	let running_in_big_picture = steamos::is_running_deck_big_picture_mode();
	let reset_default_value_on_exit = !old_fullscreen_value.is_empty() || old_fullscreen_value == "false";
	if steamos::is_running_on_steamos() && override_steamos_check.is_some() {
		help!("Running in SteamOS, support is experimental and may not work correctly.");
	}
	if running_in_big_picture || override_steamos_check.is_some() {
		status!("Applejuice is running in Big Picture mode, forcing fullscreen...");
		
		if old_fullscreen_value == "false" {
			steamos::set_rbx_fullscreen_value(true);
		} else if old_fullscreen_value.is_empty() {
			warning!("Cannot force fullscreen, returned XML value is empty; please restart Roblox to ensure the XML configuration file has been generated (Normal on first launch)");
		}
	}

	status!("Launching Roblox...");
	if binary == "Studio" { // Do a check if DataModelPatch.rbxm exists, if not, Studio might have issues when running (See: https://devforum.roblox.com/t/no-verified-patch-could-be-loaded/1797937/42?u=waviestballoon)
		let data_model_patch_path = format!("{}/ExtraContent/models/DataModelPatch/DataModelPatch.rbxm", install_path);
		help!("{}", data_model_patch_path);
		if !Path::new(&data_model_patch_path).exists() {
			warning!("DataModelPatch.rbxm does not exist, Studio may have issues when running!");
			create_notification("dialog-warning", 30000, "Missing critical file", "DataModelPatch.rbxm does not exist, Studio may have issues when running! Reinstalling Studio may fix this issue");
		}
	}
	if studio_oauthing {
		create_notification(
			&format!("{}/assets/studio.png", dir_location),
			5000,
			"Studio OAuth",
			"Launching Studio to authenticate...",
		);
	} else {
		create_notification(
			&format!("{}/assets/crudejuice.png", dir_location),
			5000,
			&format!("Roblox {} is starting!", binary),
			"",
		);
	}

	let proton_installs = configuration::get_config("proton_installations");
	let proton_installation_path = proton_installs[found_installation["preferred_proton"].as_str().unwrap_or_default()].as_str().unwrap_or_default();
	help!("Using Proton path from `preferred_proton` match: {}", proton_installation_path);
	if setup::confirm_existence(&proton_installation_path) {
		error!("Proton installation does not exist, check your configuration file");
		create_notification("dialog-warning", 30000, "Proton configuration error", "Unable to find the Proton installation to launch Roblox with, please check your configuration file to ensure that `preferred_proton` is set correctly");
		process::exit(1);
	}
	let output = process::Command::new(dbg!(format!("{}/proton", proton_installation_path)))
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
			create_notification(&format!("{}/assets/studio.png", dir_location), 5000, "Studio OAuth", "You should now be logged into Roblox Studio successfully!");
		} else {
			create_notification(&format!("{}/assets/studio.png", dir_location), 5000, "Studio OAuth",&format!("An error may have occured during the authentication process, please try again.\nExit code: {}", exitcode));
		}
	} else {
		create_notification(&format!("{}/assets/crudejuice.png", dir_location), 5000, &format!("Roblox {} has closed", binary), &format!("Exit code: {}", exitcode));
	}

	if (running_in_big_picture || override_steamos_check.is_some()) && reset_default_value_on_exit {
		status!("Fullscreen was forced; restoring previous fullscreen XML value...");
		steamos::set_rbx_fullscreen_value(old_fullscreen_value.parse::<bool>().unwrap());
	}
}
