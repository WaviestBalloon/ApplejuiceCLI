use std::fs;
use crate::utils::terminal::*;

pub fn confirm_applejuice_data_folder_existence() -> bool { // Check whether the .applejuice data folder exists under $HOME/.applejuice
	let path = format!("{}/.local/share/applejuice", env!("HOME"));

	match fs::metadata(path.clone()) {
		Ok(_) => {
			return true;
		},
		Err(_) => {
			return false;
		}
	}
}

pub fn construct_applejuice_data_folder() { // Construct the .applejuice data folder, part of initialisation
	let path: String = format!("{}/.local/share/applejuice", env!("HOME"));
	status(format!("Creating the Applejuice data directory at '{}'", path));

	match fs::create_dir(path.clone()) {
		Ok(_) => { },
		Err(errmsg) => {
			if errmsg.kind() == std::io::ErrorKind::AlreadyExists {
				warning(format!("The Applejuice data directory already exists at '{}', skipping directory construction...", path));
			} else {
				error(format!("Failed to create the Applejuice data directory, raw: '{}'\nError: {}", path, errmsg));
			}
		}
	}

	fs::write(format!("{}/{}", path, "README.txt"), "Hey! Welcome to the cool zone...\n\n\tIf you want a fresh start, delete this folder and Applejuice will forget everything!").expect("Failed to create the README file!");
	fs::write(format!("{}/{}", path, "config.json"), "{}").expect("Failed to create the config file!");
}

pub fn confirm_existence(providedpath: &str) -> bool { // Check whether a item exists in the .applejuice data folder or a ancestor to it
	let mut path = format!("{}/.local/share/applejuice/{}", env!("HOME"), providedpath);
	if providedpath.contains(get_applejuice_dir().to_string().as_str()) { // Sometimes we provide the EXACT path, so we need to check for that and overwrite the other exact path
		path = providedpath.to_string();
	}

	match fs::metadata(path.clone()) {
		Ok(_) => {
			return true;
		},
		Err(_) => {
			return false;
		}
	}
}

pub fn create_dir(providedpath: &str) -> bool { // Create a directory in the .applejuice data folder or a ancestor to it
	let mut path = format!("{}/.local/share/applejuice/{}", env!("HOME"), providedpath);
	if providedpath.contains(&get_applejuice_dir()) { // Sometimes we provide the EXACT path, so we need to check for that and overwrite the other exact path
		path = providedpath.to_string();
	}

	match fs::create_dir_all(path.clone()) {
		Ok(_) => {
			return true;
		},
		Err(_) => {
			return false;
		}
	}
}

pub fn get_applejuice_dir() -> String { // Returns where the .applejuice data folder *should* be
	return format!("{}/.local/share/applejuice", env!("HOME"));
}
