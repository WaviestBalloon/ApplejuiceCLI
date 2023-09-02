use std::{fs, env::var, process::{self, exit}};
use serde_json::json;

use crate::utils::{terminal::*, installation, setup, configuration, argparse};

const HELP_TEXT: &str = "\nUsage: --install [type] [?removeolder] [?migratefflags] \nInstalls Roblox Client or Roblox Studio\n\nOptions:\n\tclient\tInstalls the Roblox Client\n\tstudio\tInstalls Roblox Studio\n\nExample: --install client zcanary --removeolder --migratefflags";

fn download_and_install(version_hash: &str, channel: &str, threading: bool) {
	status!("Resolving package manifest for version hash {}...", version_hash);
	let package_manifest = installation::get_package_manifest(version_hash.to_string(), channel.to_string());
	success!("Obtained rbxPkgManifest.txt successfully");

	status!("Parsing package manifest...");
	let mut package_manifest_parsed: Vec<&str> = package_manifest.split('\n').collect();
	package_manifest_parsed.remove(package_manifest_parsed.len() - 1); // Remove last element which is an empty string
	let binary_type = installation::get_binary_type(package_manifest_parsed);
	status!("Discovered Binary type: {}", binary_type);

	let folder_path = format!("{}/roblox/{}/{}/{}", setup::get_applejuice_dir(), channel, binary_type, version_hash);
	if setup::create_dir(&folder_path) {
		status!("Constructed install directory at '{}'", folder_path);
	} else {
		error!("Failed to create directory at '{}'", folder_path);
		exit(1);
	}
	
	status!("Writing AppSettings.xml...");
	installation::write_appsettings_xml(folder_path.clone());
	status!("Downloading Roblox...");
	let cache_path = installation::download_deployment(binary_type, version_hash.to_string(), channel);
	success!("Downloaded deployment successfully!");

	status!("Installing Roblox...");
	installation::extract_deployment_zips(binary_type, cache_path, folder_path.clone(), !threading);
	success!("Extracted deployment successfully!");

	status!("Creating ClientSettings for FFlag configuration...");
	if !setup::confirm_existence(format!("{}/ClientSettings", folder_path).as_str()) {
		fs::create_dir(format!("{}/ClientSettings", folder_path)).expect("Failed to create ClientSettings directory");
		fs::write(format!("{}/ClientSettings/ClientAppSettings.json", folder_path), json!({}).to_string()).expect("Failed to create the config file!");
	} else {
		warning!("Not creating ClientSettings directory as it already exists!");
	}

	status!("Reading available Proton instances from configuration...");
	let proton_instances = configuration::get_config("proton_installations");
	let mut proton_instance: String = "".to_string();
	if !proton_instances.is_null() {
		for (_key, value) in proton_instances.as_object().unwrap() {
			if value.as_str().unwrap().contains("Proton") {
				proton_instance = value.as_str().unwrap().to_string();
				break;
			}
		}
		success!("Setting \"preferred_proton\" to {}", proton_instance);
	} else {
		warning!("Failed to find a Proton instance! Do you have one specified in your config.json file?");
	}

	status!("Creating application shortcut...");
	let clean_version_hash = version_hash.replace("version-", "");
	let desktop_shortcut_path = format!("{}/.local/share/applications/roblox-{}-{}.desktop", var("HOME").expect("$HOME not set"), binary_type.to_lowercase(), clean_version_hash);
	let desktop_shortcut_contents = format!("[Desktop Entry]
Name=Roblox {binary_type} ({channel}-{clean_version_hash})
Comment=Launch Roblox with Proton
Exec=env applejuicecli --launch --binary {binary_type} --channel {channel} --hash {version_hash} --args %u
Icon={folder_path}/content/textures/loading/robloxTilt.png
Type=Application
Categories=Game
MimeType=x-scheme-handler/{}", if binary_type == "Studio" { "roblox-studio;x-scheme-handler/roblox-studio-auth" } else { "roblox-player" });
	fs::write(desktop_shortcut_path.clone(), desktop_shortcut_contents).expect("Failed to write desktop shortcut");

	status!("Updating desktop database...");
	process::Command::new("update-desktop-database")
		.arg(format!("{}/.local/share/applications", var("HOME").expect("$HOME not set")))
		.spawn()
		.expect("Failed to execute update-desktop-database");

	configuration::update_config(serde_json::json!({
		format!("{}", version_hash): {
			"version": version_hash,
			"channel": channel,
			"binary_type": binary_type,
			"install_path": folder_path,
			"shortcut_path": desktop_shortcut_path,
			"preferred_proton": proton_instance
		}
	}), version_hash);

	success!("Roblox {} has been installed!\n\t{} {} located in {}", binary_type, binary_type, version_hash, folder_path);
}

pub fn main(args: Vec<Vec<(String, String)>>) {
	let binding = argparse::get_param_value(args.clone(), "install");
	let parsed_args = binding.split(' ').collect::<Vec<&str>>();
	if parsed_args.is_empty() || parsed_args[0] == "blank" {
		error!("No command line arguments provided for install!{}", HELP_TEXT);
		exit(1);
	}
	let install_type: &str = match parsed_args[0] {
		"studio" => "Studio",
		"player" => "Player",
		_ => {
			error!("Unknown type to install '{}'{}", parsed_args[0], HELP_TEXT);
			exit(1);
		}
	};

	let threading = !argparse::get_param_value(args, "nothreads").is_empty();
	let channel = parsed_args.get(1).map_or("LIVE", |&channel| channel);
	let version_hash = parsed_args.get(2).map_or_else(|| {
		status!("Getting latest version hash...");
		installation::get_latest_version_hash(install_type, channel)
	}, |&string| string.to_owned());

	// TODO: use these flags
	// let _remove_older = argparse::get_param_value(raw_args.clone(), "removeolder").is_empty();
	// let _migrate_fflags = argparse::get_param_value(raw_args.clone(), "migratefflags").is_empty();

	match install_type {
		"Player" => download_and_install(&version_hash, &channel, threading),
		"Studio" => download_and_install(&version_hash, &channel, threading),
		_ => unreachable!()
	}
}
