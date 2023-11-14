use std::{fs, env::var, process::exit, time};
use serde_json::json;
use sdl2::init;

use crate::utils::{terminal::*, installation::{self, ExactVersion, Version}, setup, configuration, argparse::get_param_value_new};

const HELP_TEXT: &str = "\nUsage: --install <hash | binary> [channel] [--exact] [--removeolder] [--migratefflags]\nInstalls the Roblox Player or Roblox Studio\n\nbinary:\n\tPlayer\tInstalls the Roblox Player\n\tStudio\tInstalls Roblox Studio\n\nExample: --install player zcanary --removeolder --migratefflags";

const ROBLOX_PLAYER_MIMES: &str = "x-scheme-handler/roblox-player;x-scheme-handler/roblox";
const ROBLOX_STUDIO_MIMES: &str = "x-scheme-handler/roblox-studio;x-scheme-handler/roblox-studio-auth";

fn detect_display_hertz() -> i32 {
	match init() {
		Ok(sdl_context) => {
			let display_mode = sdl_context.video().unwrap().desktop_display_mode(0).unwrap();
			display_mode.refresh_rate
		},
		Err(_) => {
			warning!("Failed to detect display refresh rate! Defaulting to 60Hz");
			60
		}
	}
}

fn download_and_install(version: ExactVersion, threading: bool) {
	let start_time = time::Instant::now();
	let global_config = configuration::get_config("global");
	let overrides = global_config["misc"]["overrides"].clone();
	let remove_deployment_postinstall = global_config["misc"]["purge_cache_deployment_after_install"].clone();

	let ExactVersion {hash: version_hash, channel} = version;

	let indentation = status!("Fetching package manifest...");
	let package_manifest = installation::get_package_manifest(version_hash.to_string(), channel.to_string());
	success!("Done");
	drop(indentation);

	let mut package_manifest_parsed: Vec<&str> = package_manifest.split('\n').collect();
	package_manifest_parsed.remove(package_manifest_parsed.len() - 1); // Remove last element which is an empty string
	let binary_type = installation::get_binary_type(package_manifest_parsed);

	let indentation = status!("Downloading deployment...");
	let cache_path = installation::download_deployment(binary_type, version_hash.to_string(), &channel);
	success!("Done");
	drop(indentation);

	let indentation = status!("Extracting deployment...");
	let folder_path = format!("{}/roblox/{}/{}/{}", setup::get_applejuice_dir(), channel, binary_type, version_hash);
	if setup::create_dir(&folder_path) {
		success!("Constructed install directory at {:?}", folder_path);
	} else {
		error!("Failed to create directory at '{}'", folder_path);
		exit(1);
	}
	status!("Writing AppSettings.xml...");
	installation::write_appsettings_xml(folder_path.clone());
	status!("Extracting deployment...");
	installation::extract_deployment_zips(binary_type, cache_path.clone(), folder_path.clone(), !threading);
	success!("Done");
	drop(indentation);

	status!("Carrying out post-install tasks... (Cleanup, FFlag configuration, etc)");

	status!("Creating ClientSettings for FFlag configuration...");
	if !setup::confirm_existence(format!("{}/ClientSettings", folder_path).as_str()) {
		let display_refresh_rate = detect_display_hertz();
		let mut target_fps = display_refresh_rate;
		if target_fps <= 100 { target_fps *= 2 }
		status!("Setting target FPS to {} based on your display refresh rate of {}Hz", target_fps, display_refresh_rate);
		fs::create_dir(format!("{}/ClientSettings", folder_path)).expect("Failed to create ClientSettings directory");
		fs::write(format!("{}/ClientSettings/ClientAppSettings.json", folder_path), json!({
			"FLogNetwork": 7, // Level 7 logs prints and thingys, needed for BloxstrapRPC integration
			"DFIntTaskSchedulerTargetFps": target_fps, // Uncapping FPS to the monitor's refresh rate
		}).to_string()).expect("Failed to create the config file!");
	} else {
		warning!("Not creating ClientSettings directory as it already exists!");
	}

	status!("Finding available Proton instances from configuration...");
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
MimeType={}", if binary_type == "Studio" { ROBLOX_STUDIO_MIMES } else { ROBLOX_PLAYER_MIMES });
	fs::write(desktop_shortcut_path.clone(), desktop_shortcut_contents).expect("Failed to write desktop shortcut");

	if remove_deployment_postinstall == true {
		status!("Deleting cached deployment...");
		fs::remove_dir_all(cache_path).expect("Failed to remove deployment from cache post-install!");
	}

	configuration::update_desktop_database();

	configuration::update_config(serde_json::json!({
		binary_type: {
			"channel": channel,
			"version_hash": version_hash,
			"install_path": folder_path,
			"shortcut_path": desktop_shortcut_path,
			"preferred_proton": proton_instance,
			"configuration": {
				"preferred_compat": "proton",
				"parameters": "%command%",
				"enable_rpc": true
			}
		}
	}), "roblox_installations");

	success!("Roblox {} has been installed!\n\t{} {} located in {}\n\tTook {:?} to complete", binary_type, binary_type, version_hash, folder_path, start_time.elapsed());
}

pub fn main(arguments: &[(String, String)]) {
	// Retrieve Arguments
	let Some(mut inline_arguments) = get_param_value_new(arguments, "install")
		.map(|inline_arguments| inline_arguments.split(' '))
			else {unreachable!("caller must ensure --install is always present")};
	let exact = get_param_value_new(arguments, "exact").is_some();
	let threading = get_param_value_new(arguments, "nothreads").is_none();
	// TODO: use these flags
	// let _remove_older = argparse::get_param_value(raw_args.clone(), "removeolder").is_empty();
	// let _migrate_fflags = argparse::get_param_value(raw_args.clone(), "migratefflags").is_empty();

	// Process Arguments
	let Some(hash_or_binary) = inline_arguments.next()
		.filter(|hash_or_binary| !hash_or_binary.is_empty())
			else {error!("{}", HELP_TEXT); exit(1)};
	let channel = inline_arguments.next().unwrap_or("LIVE");
	if inline_arguments.next().is_some() {error!("{}", HELP_TEXT); exit(1)}
	let version = match exact {
		true => Version::exact(channel, hash_or_binary),
		false => Version::latest(channel, hash_or_binary)
	};

	// Download
	download_and_install(version.fetch_latest(), threading)
}
