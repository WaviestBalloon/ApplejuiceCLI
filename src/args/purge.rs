use crate::utils::{setup, terminal::*, argparse, configuration::{get_config, update_config, self}};
use std::{fs::{read_dir, remove_dir_all, remove_file, metadata}, process, env::var};

static ACCEPTED_PARAMS: [(&str, &str); 2] = [
	("cache", "Deletes all cached deployments (~/.local/share/applejuice/cache)"),
	("installs", "Uninstalls Roblox Player and Roblox Studio")
];

fn check_location(folder: &str) {
	if !setup::confirm_existence(folder) {
		error!("{folder} directory does not exist! Did you forget to initialise?");
		process::exit(1);
	}

	let paths = read_dir(format!("{}/cache", setup::get_applejuice_dir())).unwrap();
	if paths.count() == 0 {
		error!("{folder} directory is empty!");
		process::exit(1);
	}
}

pub fn main(args: Vec<Vec<(String, String)>>) {
	let binding = argparse::get_param_value(args, "purge");
	let parsed_args = binding.split(" ").collect::<Vec<&str>>();

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

			status!("Removing shortcuts to deployments...");
			removing.iter().for_each(|version| {
				let config = get_config(version);
				let binary_type = config.get("binary_type").unwrap().as_str().unwrap();
				let clean_version_hash = version.replace("version-", "");
				let desktop_shortcut_path = format!("{}/.local/share/applications/roblox-{}-{}.desktop", var("HOME").expect("$HOME not set"), binary_type.to_lowercase(), clean_version_hash);
				if metadata(desktop_shortcut_path.clone()).is_ok() {
					remove_file(desktop_shortcut_path).unwrap();
				}
			});
			
			configuration::update_desktop_database();

			status!("Removing installation entires from configuration file...");
			removing.iter().for_each(|version| {
				update_config(serde_json::Value::Null, version);
			});

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
