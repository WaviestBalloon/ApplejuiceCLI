use crate::utils::{setup, steamos, terminal::*};
use std::fs;

pub fn main() {
	status!("Fetching system information...");

	println!("Version: {}", env!("CARGO_PKG_VERSION"));
	println!("Application directory: {}", setup::get_applejuice_dir());
	println!("Data folder exists? {}", setup::confirm_applejuice_data_folder_existence());
	println!("Running on SteamOS? {}", steamos::is_running_on_steamos());
	println!("/etc/os-release: {:?}", steamos::parse_osrelease().unwrap_or_default());
	println!("Configuration file data: {}", fs::read_to_string(format!("{}/config.json", setup::get_applejuice_dir())).unwrap());

	success!("Finished, copy and paste everything above this line!");
}
