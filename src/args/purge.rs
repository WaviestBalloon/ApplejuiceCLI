use crate::utils::{setup, terminal::*, argparse, configuration::{get_config, update_config}};
use std::{fs::{read_dir, remove_dir_all, remove_file, metadata}, process::exit, env::var, process};

const HELP_TEXT: &str = "\nUsage: --purge [type]\nPurges cache or installs, useful for a fresh start or if you are having issues\n\nOptions:\n\tcache\tDeletes all compressed files that were downloaded from the CDN\n\tinstalls\tNukes every install of Roblox you have\n\tinstall\tDeletes a specific version of Roblox, can purge multiple versions at once";

fn check_location(folder: &str) {
	if !setup::confirm_existence(folder) {
		error!("{folder} directory does not exist! Did you forget to initialise?");
		exit(1);
	}

	let paths = read_dir(format!("{}/cache", setup::get_applejuice_dir())).unwrap();
	if paths.count() == 0 {
		error!("{folder} directory is empty!");
		exit(1);
	}
}

fn get_all_valid_installations() {
	// Vec format: (location, size, installation_date, special_mods)
	let mut valid_player_versions: Vec<(String, String, String, String)> = vec![];
	let mut valid_studio_versions: Vec<(String, String, String, String)> = vec![];

	let paths = read_dir(format!("{}/roblox", setup::get_applejuice_dir())).unwrap();
	for deployment_path in paths { // "roblox/<DEPLOYMENT_CHANNEL>"
		let studio_player = read_dir(deployment_path.unwrap().path()).unwrap();

		for binary_type in studio_player { // "roblox/<DEPLOYMENT_CHANNEL>/<STUDIO/PLAYER>"
			let version = read_dir(binary_type.unwrap().path()).unwrap();

			for version_path in version { // "roblox/<DEPLOYMENT_CHANNEL>/<STUDIO/PLAYER>/<VERSION>"
				//let path_unwrap = version_path.unwrap().path().to_str().unwrap();

				//println!("{}", path_unwrap);
				println!("{:?}", version_path.unwrap().path());
			}
		}
	}
}

pub fn main(args: Vec<Vec<(String, String)>>) {
	let binding = argparse::get_param_value(args, "purge");
	let parsed_args = binding.split(' ').collect::<Vec<&str>>();
	if parsed_args[0].is_empty() {
		error!("No command line arguments provided to purge!");
		eprintln!("{}", HELP_TEXT);
		exit(1);
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
					exit(1);
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
														exit(1);
													}
												}
											}
										},
										Err(errmsg) => {
											error!("Failed to read the Roblox directory!\nError: {}", errmsg);
											exit(1);
										}
									}
								}
							},
							Err(errmsg) => {
								error!("Failed to read the Roblox directory!\nError: {}", errmsg);
								exit(1);
							}
						}
					}
				},
				Err(errmsg) => {
					error!("Failed to read the Roblox directory!\nError: {}", errmsg);
					exit(1);
				}
			}

			status!("Cleaning up...");
			match remove_dir_all(format!("{}/roblox", setup::get_applejuice_dir())) {
				Ok(_) => {
					success!("Removed Roblox directory");
				},
				Err(errmsg) => {
					format!("Failed to remove the Roblox directory!\nError: {}", errmsg);
					exit(1);
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
			status!("Updating desktop database...");
			process::Command::new("update-desktop-database")
				.arg(format!("{}/.local/share/applications", var("HOME").expect("$HOME not set")))
				.spawn()
				.expect("Failed to execute update-desktop-database");

			status!("Removing installation entires from configuration file...");
			removing.iter().for_each(|version| {
				update_config(serde_json::Value::Null, version);
			});

			setup::create_dir("roblox");
			success!("Created Roblox directory");

			success!("Purged Roblox installations successfully");
		},
		"install" => {
			status!("Discovering Roblox installations...");
			check_location("roblox");
			
			get_all_valid_installations();

			error!("Not implemented yet!");
		},
		_ => {
			error!("Unknown type to purge {:?}", parsed_args[0]);
			eprintln!("{}", HELP_TEXT);
			exit(1);
		}
	}
}
