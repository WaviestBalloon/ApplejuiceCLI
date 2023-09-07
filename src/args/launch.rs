use crate::utils::{setup, terminal::*, argparse, installation, notification::create_notification};
use crate::configuration;
use std::fs;
use std::io::Read;
use std::{process, thread, time, ffi::OsStr};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use inotify::{Inotify, WatchMask, EventMask};
use serde_json::json;

const _HELP_TEXT: &str = "\nUsage: TODO";

pub fn main(raw_args: &[(String, String)]) {
	let dir_location = setup::get_applejuice_dir();
	let binary_type = argparse::get_param_value_new(&raw_args, "binary").unwrap();
	let channel = argparse::get_param_value_new(&raw_args, "channel").unwrap();
	let version_hash = argparse::get_param_value_new(&raw_args, "hash").unwrap();
	let protocol_arguments = argparse::get_param_value_new(&raw_args, "args").unwrap();
	let skip_update_check = argparse::get_param_value_new(&raw_args, "skipupdatecheck"); // Optional
	let debug_notifications = argparse::get_param_value_new(&raw_args, "debug"); // Optional
	
	if skip_update_check.is_none() {
		status!("Checking for updates...");
		let latest_version = installation::get_latest_version_hash(binary_type, channel);

		if &latest_version == version_hash {
			success!("You are on the latest version!");
		} else {
			warning!("You are not on the latest version! You are on {} and the latest version for {} is {}", version_hash, channel, latest_version);
			let formatted_install_command = format!("--install {} {}", if binary_type == "Player" { "client" } else { "studio" }, if channel == "LIVE" { "" } else { channel });
			create_notification("dialog-warning", "5000", "Version outdated!", &format!("You are on {} and the latest version for {} is {}\nConsider running \"{}\"", version_hash.replace("version-", ""), channel, latest_version.replace("version-", ""), formatted_install_command));
		}
	}
	status!("Protocol parameter(s): {}", protocol_arguments);
	if debug_notifications.is_some() { create_notification("dialog-info", "15000", "Debug protocol parameters", protocol_arguments); }

	status!("Detecting Proton...");
	let installation_configuration = configuration::get_config(version_hash);
	let installed_deployment_location = installation_configuration["install_path"].as_str().unwrap();
	
	status!("Starting RPC...");
	let client = match DiscordIpcClient::new("1145934604444897410").and_then(|mut client| {
		client.connect()?;

		let state = format!("Using Roblox {} on Linux!", binary_type.clone());
		let payload = activity::Activity::new()
			.state(&state)
			.details("With Applejuice")
			.assets(activity::Assets::new()
				.large_image("holy_fuck_juice")
				.large_text("Bitdancer Approved"))
			.timestamps(activity::Timestamps::new()
				.start(time::SystemTime::now()
					.duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64));

		client.set_activity(payload)?;
		success!("RPC instance started");
		if debug_notifications.is_some() { create_notification("dialog-info", "15000", "Debug RPC", "Rich presence connected"); }

		// TODO: Get latest log file and tail stdout it for BloxstrapRPC
		thread::spawn(|| {
			let mut inotify = Inotify::init().expect("Failed to initialise inotify");
			let log_directory = format!("{}/prefixdata/pfx/drive_c/users/steamuser/AppData/Local/Roblox/logs/", setup::get_applejuice_dir());
			let mut buffer = [0; 1024];
			
			inotify.watches().add(log_directory.clone(), WatchMask::CREATE).expect("Error adding watch");
			status!("Waiting for log file on separate thread...");

			let mut event = inotify.read_events_blocking(&mut buffer).expect("Failed to read_events");
			let file = loop {
				match event.next() {
					Some(x) => {
						let filename = x.name.unwrap().to_string_lossy();
						if filename.contains("last.log") {
							inotify.watches().remove(x.wd).expect("Error removing watch");
							break Some(filename)
						}
					},
					None => break None
				}
			};
			if let Some(file) = file {
				success!("Log found: {file} was created - {log_directory}{file}");
				inotify.watches().add(format!("{log_directory}{file}"), WatchMask::ACCESS).expect("Error adding watch to tail log file");
				
				let mut event = inotify.into_event_stream(&mut buffer).expect("Failed to into_event_stream");
			} else {
				warning!("A file was created, but it is not a log file");
			}
		});

		Ok(client)
	}) {
		Ok(client) => Some(client),
		Err(errmsg) => {
			warning!("Failed to start RPC instance");
			if debug_notifications.is_some() { create_notification("dialog-info", "15000", "Debug RPC", &format!("Rich presence failed to start!\n{}", errmsg)); }
			None
		}
	};

	status!("Launching Roblox...");
	create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Roblox {} is starting!", binary_type), "");
	let output = process::Command::new(dbg!(format!("{}/proton", installation_configuration["preferred_proton"].as_str().unwrap())))
		.env("STEAM_COMPAT_DATA_PATH", format!("{}/prefixdata", dir_location))
		.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", format!("{}/not-steam", dir_location))
		.arg("waitforexitandrun")
		.arg(format!("{}/{}", installed_deployment_location, if binary_type == "Player" { "RobloxPlayerBeta.exe".to_string() } else { "RobloxStudioBeta.exe".to_string() }))
		.arg(protocol_arguments)
		.spawn()
		.expect("Failed to launch Roblox Player using Proton")
		.wait()
		.expect("Failed to wait on Roblox Player using Proton");

	status!("Roblox has exited with code {}", output.code().unwrap_or(0));
	create_notification(&format!("{}/assets/crudejuice.png", dir_location), "5000", &format!("Roblox {} has closed", binary_type), &format!("Exit code: {}", output.code().unwrap_or(0)));
	
	status!("Dropping RPC...");
	drop(client);
}
