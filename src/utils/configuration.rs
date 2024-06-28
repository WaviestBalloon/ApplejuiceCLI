use std::{fs, process, env::var, collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};
use crate::utils::{terminal::*, setup};

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
	preferred_proton: &'a Path,
	shortcut_path: &'a Path,
	version: &'a str
}

/*pub fn test() {
	let mut x = Config::default();
	x.proton_installations.insert("test_key", Path::new("/pathtest"));
	let a = serde_json::to_string(&x).unwrap();
	let y: Config = serde_json::from_str(&a).unwrap();
	println!("{:?}", y);
}*/

// Update a certain element in the configuration JSON file
pub fn update_config(json: serde_json::Value, config_type: &str) {
	let config_path = format!("{}/config.json", setup::get_applejuice_dir());
	if !setup::confirm_existence(&config_path) {
		warning!("Failed to find configuration file at '{}', calling to construct the data folder!", config_path);
		setup::construct_applejuice_data_folder();
	}
	
	let config_file = fs::read_to_string(config_path.clone()).unwrap();
	let mut config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

	// do not overwrite everything inside of config_type, just append onto it
	let mut config_type_json = config_json[config_type].clone();
	for (key, value) in json.as_object().unwrap() {
		config_type_json[key] = value.clone();
	}
	config_json[config_type] = config_type_json;


	//config_json[config_type] = json;

	//serde_json::from_str::<Config>(&config_file)
	fs::write(config_path, serde_json::to_string_pretty(&config_json).unwrap()).unwrap();
}

// Fetch element from configuration JSON file
pub fn get_config(config_type: &str) -> serde_json::Value {
	let config_path = format!("{}/config.json", setup::get_applejuice_dir());
	if !setup::confirm_existence(&config_path) {
		warning!("Failed to find configuration file at '{}', calling to construct the data folder!", config_path);
		setup::construct_applejuice_data_folder();
	}

	let config_file = fs::read_to_string(config_path.clone()).unwrap();
	let config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

	config_json[config_type].clone()
}

// Update desktop database which is used for protocols for browser launching
pub fn update_desktop_database() -> bool {
	status!("Updating desktop database...");
	
	match process::Command::new("update-desktop-database").arg(format!("{}/.local/share/applications", var("HOME").expect("$HOME not set"))).spawn() {
		Ok(_) => true,
		Err(_) => {
			warning!("Failed to update desktop database; protocols might not work correctly!");
			false
		}
	}
}
