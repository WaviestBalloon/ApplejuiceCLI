use std::{fs, process, time};
use serde_json::json;
use sdl2::init;

use crate::utils::{terminal::*, installation::{self, ExactVersion, Version}, setup, configuration, argparse::get_param_value_new, argparse};

static ACCEPTED_PARAMS: [(&str, &str); 2] = [
	("binary", "Player or Studio"),
	("--migratefflags", "Copy FFlag configuration from the Roblox installation to the new one")
];

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
	let _overrides = global_config["misc"]["overrides"].clone();
	let remove_cache_deployment_postinstall = global_config["misc"]["purge_cached_deployment_after_install"].clone();
	let remove_deployment_postinstall = global_config["misc"]["purge_cached_deployment_after_install"].clone();

	let ExactVersion {hash: version_hash, channel} = version;

	let indentation = status!("Fetching package manifest...");
	let package_manifest = installation::get_package_manifest(version_hash.to_string(), channel.to_string());
	success!("Done");
	drop(indentation);

	let mut package_manifest_parsed: Vec<&str> = package_manifest.split("\n").collect();
	package_manifest_parsed.remove(package_manifest_parsed.len() - 1); // Remove last element which is an empty string
	let binary_type = installation::get_binary_type(package_manifest_parsed);

	let mut are_we_upgrading = false;
	let installations = configuration::get_config("roblox_installations");
	if installations[binary_type]["version_hash"] != "null" &&installations[binary_type]["version_hash"] != version_hash.to_string() {
		are_we_upgrading = true;
	}

	let indentation = status!("Downloading deployment...");
	let cache_path = installation::download_deployment(binary_type, version_hash.to_string(), &channel);
	drop(indentation);

	let indentation = status!("Extracting deployment...");
	let folder_path = format!("{}/roblox/{}/{}/{}", setup::get_applejuice_dir(), channel, binary_type, version_hash);
	if setup::create_dir(&folder_path) {
		success!("Constructed install directory at {:?}", folder_path);
	} else {
		error!("Failed to create directory at '{}'", folder_path);
		process::exit(1);
	}
	status!("Writing AppSettings.xml...");
	installation::write_appsettings_xml(folder_path.clone());
	status!("Extracting deployment...");
	installation::extract_deployment_zips(binary_type, cache_path.clone(), folder_path.clone(), !threading);
	drop(indentation);

	status!("Carrying out post-install tasks... (Cleanup, FFlag configuration, etc)");

	if binary_type == "Studio" && setup::confirm_existence(format!("{}/StudioFonts/SourceSansPro-Black.ttf", folder_path).as_str()) {
		status!("Applying fix for broken Studio font...");
		fs::remove_file(format!("{}/StudioFonts/SourceSansPro-Black.ttf", folder_path)).expect("Failed to remove broken font");
	}

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
		for (key, value) in proton_instances.as_object().unwrap() {
			if value.as_str().unwrap().contains("Proton") {
				proton_instance = key.to_string();
				break;
			}
		}
		success!("Setting `preferred_proton` to {}", proton_instance);
	} else {
		warning!("Failed to find a Proton instance! Do you have one specified in your config.json file?");
	}

	if remove_cache_deployment_postinstall == true {
		status!("Deleting cached deployment...");
		fs::remove_dir_all(cache_path).expect("Failed to remove deployment from cache for post-install!");
	}
	if remove_deployment_postinstall == true && are_we_upgrading == true {
		status!("Deleting old deployment...");
		let old_install_location = installations[binary_type]["install_path"].to_string();

		if fs::metadata(old_install_location.clone()).is_err() {
			warning!("Failed to find old deployment, skipping removal...");
		} else {
			fs::remove_dir_all(old_install_location).expect("Failed to remove deployment from previous installation for post-install!");
		}
	}

	configuration::update_desktop_database();

	configuration::update_config(serde_json::json!({
		binary_type: {
			"channel": channel,
			"version_hash": version_hash,
			"install_path": folder_path,
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
			else {
				help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
				process::exit(1);
			};
	let channel = inline_arguments.next().unwrap_or("LIVE");
	if inline_arguments.next().is_some() {
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		process::exit(1);
	}
	let version = match exact {
		true => Version::exact(channel, hash_or_binary),
		false => Version::latest(channel, hash_or_binary)
	};

	// Download
	download_and_install(version.fetch_latest(), threading);
}
