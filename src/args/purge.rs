use crate::utils::setup;
use crate::utils::terminal::*;
use crate::utils::args;
use std::fs::{read_dir, remove_dir_all};

const HELP_TEXT: &str = "\nUsage: --purge [type]\nPurges cache or installs, useful for a fresh start or if you are having issues\n\nOptions:\n\tcache\tDeletes all compressed files that were downloaded from the CDN\n\tinstalls\tNukes every install of Roblox you have\n\tinstall\tDeletes a specific version of Roblox, can purge multiple versions at once";

pub fn main(args: Vec<Vec<(String, String)>>) {
	let binding = args::get_param_value(args, "purge");
	let parsed_args = binding.split(" ").collect::<Vec<&str>>();
	if parsed_args[0].is_empty() {
		error(format!("No command line arguments provided to purge!{}", HELP_TEXT));
	}
	let install_type: &str = &parsed_args[0];

	println!("{:?}", parsed_args.clone());

	match install_type {
		"cache" => {
			status("Purging cache...");

			if setup::confirm_existence("cache") {
				let paths = read_dir(format!("{}/cache", setup::get_applejuice_dir())).unwrap();
				if paths.count() == 0 {
					error("Cache directory is empty!");
				}

				match remove_dir_all(format!("{}/cache", setup::get_applejuice_dir())) {
					Ok(_) => {
						success("Removed cache directory");
					},
					Err(errmsg) => {
						error(format!("Failed to remove the cache directory!\nError: {}", errmsg));
					}
				}
				
				setup::create_dir("cache");
				success("Created cache directory");
			} else {
				error("Cache directory does not exist! Did you forget to initialise?");
			}

			success("Purged cache successfully");
		},
		"installs" => {
			status("Purging Roblox installations...");

			if setup::confirm_existence("roblox") {
				let paths = read_dir(format!("{}/roblox", setup::get_applejuice_dir())).unwrap();
				if paths.count() == 0 {
					error("Roblox directory is empty!");
				}

				match remove_dir_all(format!("{}/roblox", setup::get_applejuice_dir())) {
					Ok(_) => {
						success("Removed Roblox directory");
					},
					Err(errmsg) => {
						error(format!("Failed to remove the Roblox directory!\nError: {}", errmsg));
					}
				}

				status("Searching for remnant entries in config...");
				// TODO: Remove remnants from config
				
				setup::create_dir("roblox");
				success("Created Roblox directory");
			} else {
				error("Roblox directory does not exist! Did you forget to initialise?");
			}

			success("Purged Roblox installations successfully");
		},
		"install" => {
			/*status(format!("Purging Roblox installation(s) for version(s) {}...", parsed_args[1..].join(", ")));

			if setup::confirm_existence("roblox") {
				let paths = read_dir(format!("{}/roblox", setup::get_applejuice_dir())).unwrap();
				if paths.count() == 0 {
					error("Roblox directory is empty!");
				}

				for version in parsed_args[1..].iter() {
					for path in read_dir(format!("{}/roblox", setup::get_applejuice_dir())).unwrap() {
						let path_unwrap = path.unwrap();
						if path_unwrap.path().to_str().unwrap().contains(version) {
							match remove_dir_all(path_unwrap.path()) {
								Ok(_) => {
									success(format!("Removed Roblox directory for version {}", version));
								},
								Err(errmsg) => {
									error(format!("Failed to remove the Roblox directory for version {}!\nError: {}", version, errmsg));
								}
							}
						}
					}
				}

				status("Searching for remnant entries in config...");
			} else {
				error("Roblox directory does not exist! Did you forget to initialise?");
			}

			success("Purged Roblox installation(s) successfully");*/
		},
		_ => {
			error(format!("Unknown type to purge '{}'{}", parsed_args[0], HELP_TEXT));
		}
	}
}
