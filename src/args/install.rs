use std::{fs, env::var};
use crate::utils::{terminal::*, installation, setup, configuration, args};

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
		status(format!("Constructed install directory at '{}'", folder_path));
	} else {
		error(format!("Failed to create directory at '{}'", folder_path));
	}
	
	status("Writing AppSettings.xml...");
	installation::write_appsettings_xml(folder_path.clone());
	status("Downloading Roblox...");
	let cache_path = installation::download_deployment(binary_type, version_hash.to_string(), channel);
	success("Downloaded deployment successfully!");

	status("Installing Roblox...");
	installation::extract_deployment_zips(binary_type, cache_path, folder_path.clone());
	success("Extracted deployment successfully!");

	status("Updating configuration file...");
	configuration::update_config(serde_json::json!({
		format!("{}", version_hash): {
			"version": version_hash,
			"channel": channel,
			"binary_type": binary_type,
			"install_path": folder_path.to_string()
		}
	}), &version_hash);

	status("Creating application shortcut...");
	status("Reading available Proton instances from configuration...");
	let proton_instances = configuration::get_config("proton_installations");
	let mut proton_instance: String = "".to_string();
	if !proton_instances.is_null() {
		for (key, value) in proton_instances.as_object().unwrap() {
			if value.as_str().unwrap().contains("Proton") {
				proton_instance = key.to_string();
				break;
			}
		}
		success(format!("Found Proton instance '{}'", proton_instance));
	} else {
		warning("Failed to find a Proton instance! Do you have one specified in your config.json file?");
	}
	let clean_version_hash = version_hash.replace("version-", "");
	let desktop_shortcut_path = format!("{}/.local/share/applications/roblox-{}-{}.desktop", var("HOME").expect("$HOME not set"), binary_type.to_lowercase(), clean_version_hash);
	let desktop_shortcut_contents = format!("[Desktop Entry]
Name=Roblox {binary_type} ({channel}-{clean_version_hash})
Comment=Launch Roblox with Proton
Exec=env notify-send \"Launching Roblox {binary_type}\"
Icon={folder_path}/content/textures/loading/robloxTilt.png
Type=Application
Categories=Game;");
	fs::write(desktop_shortcut_path, desktop_shortcut_contents).expect("Failed to write desktop shortcut");

	success(format!("Roblox {} has been installed!\n\t{} {} located in {}", binary_type, binary_type, version_hash, folder_path));
}

fn install_client(channel_arg: Option<String>, version_hash_arg: Option<String>) {
	let version_hash: String;
	let mut channel: String = "LIVE".to_string();
	let mut _protocol: bool = false;
	let mut _uncap_fps_fflag: bool = false;
	
	if !channel_arg.is_some() {
		status("Defaulting to LIVE channel...");
	} else {
		channel = channel_arg.unwrap_or_else(|| "LIVE".to_string());
		status(format!("Using channel: {}", channel));
	}
	if !version_hash_arg.is_some() {
		status("Getting latest version hash...");
		version_hash = installation::get_latest_version_hash("Player", &channel);
	} else {
		version_hash = version_hash_arg.unwrap();
	}

	download_and_install(&version_hash, &channel);
}
fn install_studio(channel_arg: Option<String>, version_hash_arg: Option<String>) {
	if !version_hash_arg.is_some() {
		warning("No version hash provided, getting latest version hash instead...");
	}
	let channel: String = channel_arg.unwrap_or_else(|| "LIVE".to_owned());
	let version_hash: String = version_hash_arg.unwrap_or_else(|| installation::get_latest_version_hash("Studio", &channel));

	download_and_install(&version_hash, "LIVE");
}

pub fn main(args: Vec<Vec<(String, String)>>) {
	let binding = args::get_param_value(args, "install");
	let parsed_args = binding.split(" ").collect::<Vec<&str>>();
	if parsed_args.len() == 0 {
		error(format!("No command line arguments provided for install!{}", HELP_TEXT));
	}
	let install_type: &str = &parsed_args[0];

	match install_type {
		"client" => install_client(parsed_args.get(1).map(|&string| string.to_owned()), parsed_args.get(2).map(|&string| string.to_owned())),
		"studio" => install_studio(parsed_args.get(1).map(|&string| string.to_owned()), parsed_args.get(2).map(|&string| string.to_owned())),
		_ => {
			error(format!("Unknown type to install '{}'{}", parsed_args[0], HELP_TEXT));
		}
	}
}
