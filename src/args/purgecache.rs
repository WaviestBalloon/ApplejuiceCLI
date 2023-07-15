use crate::utils::setup;
use crate::utils::terminal::*;
use std::fs::{read_dir, remove_dir_all};

pub fn main() {
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

	println!();
	success("Purged cache successfully");
}
