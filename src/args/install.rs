use std::{process, fs};
use crate::utils::{terminal::*, installation};
use crate::setup;

const HELP_TEXT: &str = "\nUsage: --install [type]\nInstalls Roblox Client or Roblox Studio\n\nOptions:\n\tclient\tInstalls the Roblox Client\n\tstudio\tInstalls Roblox Studio";

fn install_client() {
	error("not implimented yet!");
	process::exit(1);
}
fn install_studio(version_hash_arg: Option<String>) {
	if version_hash_arg.is_some() == false {
		warning("No version hash provided, getting latest version hash instead...");
	}
	let channel: &str = "LIVE"; // TODO: Make this configurable
	let version_hash: String = version_hash_arg.unwrap_or_else(|| installation::get_latest_version_hash());

	println!("Resolving package manifest for version hash {}...", version_hash);
	let package_manifest = installation::get_package_manifest(version_hash.clone());
	success("Obtained rbxPkgManifest.txt successfully");

	println!("Parsing package manifest...");
	let mut package_manifest_parsed: Vec<&str> = package_manifest.split("\n").collect();
	package_manifest_parsed.remove(package_manifest_parsed.len() - 1); // Remove last element which is an empty string
	let binary_type = installation::get_binary_type(package_manifest_parsed);
	println!("Discovered Binary type: {}", binary_type);

	let folder_path = format!("{}/roblox/versions/{}/{}/{}", setup::get_applejuice_dir(), channel, binary_type, version_hash.to_string());
	match fs::create_dir_all(folder_path.clone()) { // TODO: Move this into crate::setup::create_dir
		Ok(_) => {
			success(format!("Constructed install directory at '{}'", folder_path));
		},
		Err(_) => {
			warning(format!("Failed to create directory at '{}'", folder_path));
		}
	}
	
	

	error("not implimented yet!");
}

pub fn main(parsed_args: &[String]) { // TODO: Move this func into args mods instead of utils mods
	if parsed_args.len() == 0 {
		error(format!("No command line arguments provided for install!{}", HELP_TEXT));
		process::exit(1);
	}
	let install_type: &str = &parsed_args[0];

	match install_type {
		"client" => install_client(),
		"studio" => install_studio(parsed_args.get(1).cloned()),
		_ => {
			error(format!("Unknown type to install '{}'{}", parsed_args[0], HELP_TEXT));
			process::exit(1);
		}
	}
}
