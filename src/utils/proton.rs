use std::{fs, env::var};
use crate::utils::terminal::*;
use serde_json;

pub fn discover_proton_directory() -> serde_json::Value { // Try to automatically find where Proton is installed to
	let potential_path = format!("{}/.steam/steam/steamapps/common/", var("HOME").expect("$HOME not set"));
	let mut installations = serde_json::json!({ "proton_installations": {} }); // Init

	match fs::read_dir(potential_path.clone()) {
		Ok(_) => {
			status(format!("Found Steam directory at '{}'", potential_path));

			for entry in fs::read_dir(potential_path.clone()).unwrap() {
				let unwrapped_entry = entry.unwrap();
				let path = unwrapped_entry.path();
				let fsname = path.file_name().unwrap().to_str().unwrap();

				if fsname.contains("Proton") {
					success(format!("Found '{}' at '{}'", fsname, path.to_str().unwrap()));
					installations["proton_installations"][fsname] = serde_json::Value::String(path.to_str().unwrap().to_string());
				}
			}

			return installations;
		},
		Err(_) => {
			warning(format!("Failed to find the Steam directory at '{}'", potential_path));
			return serde_json::Value::Null;
		}
	}
}
