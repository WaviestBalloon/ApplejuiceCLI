use std::process;
use std::fs;

pub fn confirm_applejuice_data_folder_existence() {
	let path = format!("{}/{}", env!("HOME"), ".applejuice");

	match fs::metadata(path.clone()) {
		Ok(_) => {},
		Err(_) => {
			println!("Warning: Could not verify the existence of the Applejuice data directory! Attempting to create it...");
			construct_applejuice_data_folder();
		}
	}
}

pub fn construct_applejuice_data_folder() {
	let path = format!("{}/{}", env!("HOME"), ".applejuice");
	println!("Creating the Applejuice data directory at '{}'", path);

	match fs::create_dir(path.clone()) {
		Ok(_) => { },
		Err(_) => {
			println!("Failed to create the Applejuice data directory, raw: '{}'", path);
			process::exit(1);
		}
	}

fs::write(format!("{}/{}", path, "README.txt"), "Hey! Welcome to the cool zone...\n\n\tIf you want a fresh start, delete this folder and Applejuice will forget everything!").expect("Failed to create the README file!");
	fs::write(format!("{}/{}", path, "config.json"), "{}").expect("Failed to create the config file!");
}
