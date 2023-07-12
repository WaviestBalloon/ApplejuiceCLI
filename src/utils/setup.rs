use std::fs;
use crate::utils::terminal::*;

pub fn confirm_applejuice_data_folder_existence() -> bool {
	let path = format!("{}/{}", env!("HOME"), ".applejuice");

	match fs::metadata(path.clone()) {
		Ok(_) => {
			return true;
		},
		Err(_) => {
			return false;
		}
	}
}

pub fn construct_applejuice_data_folder() {
	let path: String = format!("{}/{}", env!("HOME"), ".applejuice");
	println!("Creating the Applejuice data directory at '{}'", path);

	match fs::create_dir(path.clone()) {
		Ok(_) => { },
		Err(_) => {
			error(format!("Failed to create the Applejuice data directory, raw: '{}'", path));
		}
	}

	fs::write(format!("{}/{}", path, "README.txt"), "Hey! Welcome to the cool zone...\n\n\tIf you want a fresh start, delete this folder and Applejuice will forget everything!").expect("Failed to create the README file!");
	fs::write(format!("{}/{}", path, "config.json"), "{}").expect("Failed to create the config file!");
}

pub fn confirm_existence(providedpath: &str) -> bool {
	let path = format!("{}/{}/{}", env!("HOME"), ".applejuice", providedpath);

	match fs::metadata(path.clone()) {
		Ok(_) => {
			return true;
		},
		Err(_) => {
			return false;
		}
	}
}

pub fn create_dir(providedpath: &str) -> bool {
	let path: String = format!("{}/{}/{}", env!("HOME"), ".applejuice", providedpath);

	match fs::create_dir(path.clone()) {
		Ok(_) => {
			return true;
		},
		Err(_) => {
			return false;
		}
	}
}

pub fn get_applejuice_dir() -> String {
	return format!("{}/{}", env!("HOME"), ".applejuice");
}
