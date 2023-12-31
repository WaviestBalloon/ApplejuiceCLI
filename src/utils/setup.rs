use std::{fs, env::var, process::exit};
use serde_json::json;

use crate::utils::terminal::*;

pub fn confirm_applejuice_data_folder_existence() -> bool { // Check whether the .applejuice data folder exists under $HOME/.applejuice
	let path = format!("{}/.local/share/applejuice", var("HOME").expect("$HOME not set"));

	match fs::metadata(path.clone()) {
		Ok(_) => {
			true
		},
		Err(_) => {
			false
		}
	}
}

pub fn construct_applejuice_data_folder() { // Construct the .applejuice data folder, part of initialisation
	let path: String = format!("{}/.local/share/applejuice", var("HOME").expect("$HOME not set"));
	status!("Creating the Applejuice data directory at '{}'", path);

	match fs::create_dir(path.clone()) {
		Ok(_) => { },
		Err(errmsg) => {
			if errmsg.kind() == std::io::ErrorKind::AlreadyExists {
				warning!("The Applejuice data directory already exists at '{}', skipping directory construction...", path);
			} else {
				error!("Failed to create the Applejuice data directory, raw: '{}'\nError: {}", path, errmsg);
				exit(1);
			}
		}
	}

	status!("Creating README.txt...");
	fs::write(format!("{}/{}", path, "README.txt"), "Hey! Welcome to the cool zone...\n\tIf you want a fresh start, delete this folder and Applejuice will forget everything!\n\tGitHub: https://github.com/WaviestBalloon/ApplejuiceCLI\n\tKnown issues list: https://github.com/WaviestBalloon/ApplejuiceCLI/issues/1").expect("Failed to create the README file!");
	status!("Creating config.json...");
	fs::write(format!("{}/{}", path, "config.json"), json!({}).to_string()).expect("Failed to create the config file!");
}

pub fn confirm_existence(providedpath: &str) -> bool { // Check whether a item exists in the .applejuice data folder or a ancestor to it
	let mut path = format!("{}/.local/share/applejuice/{}", var("HOME").expect("$HOME not set"), providedpath);
	if providedpath.contains(get_applejuice_dir().to_string().as_str()) { // Sometimes we provide the EXACT path, so we need to check for that and overwrite the other exact path
		path = providedpath.to_string();
	}
	
	match fs::metadata(path.clone()) {
		Ok(_) => {
			true
		},
		Err(_) => {
			false
		}
	}
}

pub fn _confirm_existence_raw(providedpath: &str) -> bool { // Check whether a item exists in the .applejuice data folder or a ancestor to it
	match fs::metadata(providedpath) {
		Ok(_) => {
			true
		},
		Err(_) => {
			false
		}
	}
}

pub fn create_dir(providedpath: &str) -> bool { // Create a directory in the .applejuice data folder or a ancestor to it
	let mut path = format!("{}/.local/share/applejuice/{}", var("HOME").expect("$HOME not set"), providedpath);
	if providedpath.contains(&get_applejuice_dir()) { // Sometimes we provide the EXACT path, so we need to check for that and overwrite the other exact path
		path = providedpath.to_string();
	}

	match fs::create_dir_all(path.clone()) {
		Ok(_) => {
			true
		},
		Err(_) => {
			false
		}
	}
}

pub fn get_applejuice_dir() -> String { // Returns where the .applejuice data folder *should* be
	format!("{}/.local/share/applejuice", var("HOME").expect("$HOME not set"))
}
