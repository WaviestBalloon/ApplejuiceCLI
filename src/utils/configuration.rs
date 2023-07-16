use std::fs;
use crate::utils::terminal::*;
use crate::utils::setup;
use serde_json;

pub fn update_config(json: serde_json::Value, config_type: &str) {
	status("Updating configuration file...".to_string());
	let config_path = format!("{}/config.json", setup::get_applejuice_dir());
	if setup::confirm_existence(&config_path) == false {
		warning(format!("Failed to find configuration file at '{}', calling to construct the data folder!", config_path));
		setup::construct_applejuice_data_folder();
	}
	
	let config_file = fs::read_to_string(config_path.clone()).unwrap();
	let mut config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

	config_json[config_type] = json[config_type].clone();
	fs::write(config_path, serde_json::to_string_pretty(&config_json).unwrap()).unwrap();
}

pub fn _get_config(config_type: &str) -> serde_json::Value {
	let config_path = format!("{}/config.json", setup::get_applejuice_dir());
	if setup::confirm_existence(&config_path) == false {
		warning(format!("Failed to find configuration file at '{}', calling to construct the data folder!", config_path));
		setup::construct_applejuice_data_folder();
	}

	let config_file = fs::read_to_string(config_path.clone()).unwrap();
	let config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

	return config_json[config_type].clone();
}
