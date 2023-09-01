use std::fs;
use std::{collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};
use crate::utils::terminal::*;
use crate::utils::setup;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config<'a> {
	#[serde(borrow)]
	proton_installations: HashMap<&'a str, &'a Path>,
	roblox_installation: HashMap<&'a str, RobloxInstallation<'a>>
}

#[derive(Debug, Serialize, Deserialize)]
enum BinaryType {
	Player,
	Studio
}

#[derive(Debug, Serialize, Deserialize)]
struct RobloxInstallation<'a> {
	binary_type: BinaryType,
	channel: &'a str,
	install_path: &'a Path,
	prefferred_proton: &'a Path,
	shortcut_path: &'a Path,
	version: &'a str
}

/*
	{
		"binary_type": "Player",
		"channel": "zcanary",
		"install_path": "/home/wav/.local/share/applejuice/roblox/zcanary/Player/version-d44ee8435ee04b51",
		"preferred_proton": "/home/wav/.steam/steam/steamapps/common/Proton - Experimental",
		"shortcut_path": "/home/wav/.local/share/applications/roblox-player-d44ee8435ee04b51.desktop",
		"version": "version-d44ee8435ee04b51"
	}
*/

pub fn _test_balls() {
	let mut x = Config::default();
	x.proton_installations.insert("sex", Path::new("/balls"));
	let a = serde_json::to_string(&x).unwrap();
	let y: Config = serde_json::from_str(&a).unwrap();
	println!("{:?}", y);
}

pub fn update_config(json: serde_json::Value, config_type: &str) {
	status("Updating configuration file...");
	let config_path = format!("{}/config.json", setup::get_applejuice_dir());
	if !setup::confirm_existence(&config_path) {
		warning(format!("Failed to find configuration file at '{}', calling to construct the data folder!", config_path));
		setup::construct_applejuice_data_folder();
	}
	
	let config_file = fs::read_to_string(config_path.clone()).unwrap();
	let mut config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

	config_json[config_type] = json[config_type].clone();
	fs::write(config_path, serde_json::to_string_pretty(&config_json).unwrap()).unwrap();
}

pub fn get_config(config_type: &str) -> serde_json::Value {
	let config_path = format!("{}/config.json", setup::get_applejuice_dir());
	if !setup::confirm_existence(&config_path) {
		warning(format!("Failed to find configuration file at '{}', calling to construct the data folder!", config_path));
		setup::construct_applejuice_data_folder();
	}

	let config_file = fs::read_to_string(config_path.clone()).unwrap();
	let config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

	config_json[config_type].clone()
}
