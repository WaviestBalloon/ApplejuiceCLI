use std::{fs, env::var};
use crate::utils::terminal::*;
use serde_json;

pub fn discover_proton_directory() -> serde_json::Value { // Try to automatically find where Proton is installed to
	let potential_path = format!("{}/.steam/steam/steamapps/common", var("HOME").expect("$HOME not set"));
	let mut installations = serde_json::json!({}); // Init

	match fs::read_dir(potential_path.clone()) {
		Ok(_) => {
			success!("Found Steam directory at '{}'", potential_path);

			for entry in fs::read_dir(potential_path.clone()).unwrap() {
				let unwrapped_entry = entry.unwrap();
				let path = unwrapped_entry.path();
				let fsname = path.file_name().unwrap().to_str().unwrap();

				if fsname.contains("Proton") {
					success!("Found '{}' at '{}'", fsname, path.to_str().unwrap());
					installations[fsname] = serde_json::Value::String(format!("{}/proton", path.to_str().unwrap()));
				}
			}

			installations
		},
		Err(_) => {
			warning!("Failed to find the Steam directory at '{}'", potential_path);
			installations
		}
	}
}

/* Used for when we start supporting both Wine and Proton
pub fn construct_proton_process() {
	
}

pub fn construct_wine_process() {
	
}
*/
