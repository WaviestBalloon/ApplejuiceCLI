use crate::utils::{terminal::*, installation, setup, configuration};
use serde_json;

const HELP_TEXT: &str = "\nUsage: --install [type]\nInstalls Roblox Client or Roblox Studio\n\nOptions:\n\tclient\tInstalls the Roblox Client\n\tstudio\tInstalls Roblox Studio";

fn download_and_install(version_hash: &str, channel: &str) {
	status(format!("Resolving package manifest for version hash {}...", version_hash));
	let package_manifest = installation::get_package_manifest(version_hash.to_string(), channel.to_string());
	success("Obtained rbxPkgManifest.txt successfully");

	status("Parsing package manifest...");
	let mut package_manifest_parsed: Vec<&str> = package_manifest.split("\n").collect();
	package_manifest_parsed.remove(package_manifest_parsed.len() - 1); // Remove last element which is an empty string
	let binary_type = installation::get_binary_type(package_manifest_parsed);
	status(format!("Discovered Binary type: {}", binary_type));

	let folder_path = format!("{}/roblox/{}/{}/{}", setup::get_applejuice_dir(), channel, binary_type, version_hash.to_string());
	if setup::create_dir(&folder_path) == true {
		success(format!("Constructed install directory at '{}'", folder_path));
	} else {
		error(format!("Failed to create directory at '{}'", folder_path));
	}
	
	installation::write_appsettings_xml(folder_path.clone());
	let cache_path = installation::download_deployment(binary_type, version_hash.to_string(), channel);

	installation::extract_deployment_zips(binary_type, cache_path, folder_path.clone());

	configuration::update_config(serde_json::json!({
		format!("{}", version_hash): {
			"version": version_hash,
			"channel": channel,
			"binary_type": binary_type,
			"install_path": folder_path.to_string()
		}
	}), &version_hash);
	success("Extracted deployment successfully");
}

fn install_client(channel_arg: Option<String>, version_hash_arg: Option<String>) {
	let version_hash: String;
	let mut channel: String = "LIVE".to_string();
	warning("Roblox Player now has Byfron, anti-tamper software, as of now it is not currently impossible to play Roblox Player on Linux due to Wine being blacklisted. (This has been confirmed to be temporary)\n\tInstallation will continue as normal...");
	
	if channel_arg.is_some() == false {
		status("Defaulting to LIVE channel...");
	} else {
		channel = channel_arg.unwrap_or_else(|| "LIVE".to_string());
		status(format!("Using channel: {}", channel));
	}
	if version_hash_arg.is_some() == false {
		status("Getting latest version hash...");
		version_hash = installation::get_latest_version_hash("Player", &channel);
	} else {
		version_hash = version_hash_arg.unwrap();
	}

	download_and_install(&version_hash, &channel);
}
fn install_studio(channel_arg: Option<String>, version_hash_arg: Option<String>) {
	if version_hash_arg.is_some() == false {
		warning("No version hash provided, getting latest version hash instead...");
	}
	let _channel: &str = "LIVE"; // TODO: Make this configurable
	let version_hash: String = version_hash_arg.unwrap_or_else(|| installation::get_latest_version_hash("Studio", "LIVE")); // TODO: make studio channels allowed masdnekja;kr;owwk;a :(

	download_and_install(&version_hash, "LIVE");
}

pub fn main(parsed_args: &[String]) {
	if parsed_args.len() == 0 {
		error(format!("No command line arguments provided for install!{}", HELP_TEXT));
	}
	let install_type: &str = &parsed_args[0];

	match install_type {
		"client" => install_client(parsed_args.get(1).cloned(), parsed_args.get(2).cloned()),
		"studio" => install_studio(parsed_args.get(1).cloned(), parsed_args.get(2).cloned()),
		_ => {
			error(format!("Unknown type to install '{}'{}", parsed_args[0], HELP_TEXT));
		}
	}
}
