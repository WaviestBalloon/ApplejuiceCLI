use serde_json::json;

use crate::utils::{setup, terminal::*, argparse, configuration};
use std::{fs::{read_dir, remove_dir_all}, process};

static ACCEPTED_PARAMS: [(&str, &str); 2] = [
	("cache", "Deletes all cached deployments (~/.local/share/applejuice/cache)"),
	("installs", "Uninstalls Roblox Player and Roblox Studio")
];

fn check_location(folder: &str) {
	if !setup::confirm_existence(folder) {
		error!("{folder} directory does not exist! Did you forget to initialise?");
		process::exit(1);
	}

	let paths = read_dir(format!("{}/{}", setup::get_applejuice_dir(), folder)).unwrap();
	if paths.count() == 0 {
		error!("{folder} directory is empty!");
		process::exit(1);
	}
}

pub fn main(args: Vec<Vec<(String, String)>>) {
	let binding = argparse::get_param_value(args, "purge");
	let parsed_args = binding.split(' ').collect::<Vec<&str>>();

	if parsed_args[0] == "blank" {
		error!("No command line arguments provided to purge!");
		help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		process::exit(1);
	}
	let install_type: &str = parsed_args[0];
	
	match install_type {
		"cache" => {
			status!("Purging cache...");
			
			match remove_dir_all(format!("{}/cache", setup::get_applejuice_dir())) {
				Ok(_) => {
					success!("Removed cache directory");
				},
				Err(errmsg) => {
					error!("Failed to remove the cache directory!\nError: {}", errmsg);
					process::exit(1);
				}
			}
			
			setup::create_dir("cache");
			success!("Created cache directory");

			success!("Purged cache successfully");
		},
		"installs" => {
			status!("Purging Roblox installations...");
			check_location("roblox");

			let mut removing = Vec::new();
			match read_dir(format!("{}/roblox", setup::get_applejuice_dir())) {
				Ok(paths) => {
					for deployment_path in paths {
						let path_unwrap = deployment_path.unwrap();
						match read_dir(path_unwrap.path()) {
							Ok(paths) => {
								for binary_type in paths {
									let path_unwrap = binary_type.unwrap();
										match read_dir(path_unwrap.path()) {
										Ok(paths) => {
											for version_path in paths {
												let path_unwrap = version_path.unwrap();
												let binding = path_unwrap.path();
												removing.push(path_unwrap.file_name().into_string().unwrap());

												status!("Removing {:?}...", binding.clone());
												match remove_dir_all(binding.clone()) {
													Ok(_) => {
														success!("Removed Roblox directory for version {}", binding.to_str().unwrap());
														},
													Err(errmsg) => {
														error!("Failed to remove the Roblox directory for version {}!\nError: {}", path_unwrap.path().to_str().unwrap(), errmsg);
														process::exit(1);
													}
												}
											}
										},
										Err(errmsg) => {
											error!("Failed to read the Roblox directory!\nError: {}", errmsg);
											process::exit(1);
										}
									}
								}
							},
							Err(errmsg) => {
								error!("Failed to read the Roblox directory!\nError: {}", errmsg);
								process::exit(1);
							}
						}
					}
				},
				Err(errmsg) => {
					error!("Failed to read the Roblox directory!\nError: {}", errmsg);
					process::exit(1);
				}
			}

			status!("Cleaning up...");
			match remove_dir_all(format!("{}/roblox", setup::get_applejuice_dir())) {
				Ok(_) => {
					success!("Removed Roblox directory");
				},
				Err(errmsg) => {
					error!("Failed to remove the Roblox directory!\nError: {}", errmsg);
					process::exit(1);
				}
			}
			
			configuration::update_desktop_database();

			status!("Removing installation entires from configuration file...");
			configuration::update_config(json!({ }), "roblox_installations");

			setup::create_dir("roblox");
			success!("Created Roblox directory");

			success!("Purged Roblox installations successfully");
		}
		_ => {
			error!("Unknown type to purge {:?}", parsed_args[0]);
			help!("Accepted parameters:\n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
		}
	}
}
