use crate::utils::setup;
use crate::utils::terminal::*;
use std::fs::{read_dir, remove_dir_all};

const HELP_TEXT: &str = "\nUsage: --purge [type]\nPurges cache or installs, useful for a fresh start or if you are having issues\n\nOptions:\n\tcache\tDeletes all compressed files that were downloaded from the CDN\n\tinstalls\tNukes every install of Roblox you have";

pub fn main(parsed_args: &[String]) {
	if parsed_args.len() == 0 {
		error(format!("No command line arguments provided to purge!{}", HELP_TEXT));
	}
	let install_type: &str = &parsed_args[0];

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

				status("Searching for remnants entries in config...");
				// TODO: Remove remnants from config
				
				setup::create_dir("roblox");
				success("Created Roblox directory");
			} else {
				error("Roblox directory does not exist! Did you forget to initialise?");
			}

			success("Purged Roblox installations successfully");
		},
		_ => {
			error(format!("Unknown type to purge '{}'{}", parsed_args[0], HELP_TEXT));
		}
	}
}
