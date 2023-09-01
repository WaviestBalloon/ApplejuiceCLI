use crate::utils::setup;
use crate::utils::terminal::*;
use std::process;

const _HELP_TEXT: &str = "\nUsage: --opendata [type]\nOpens the data folder for Applejuice, where installations, cache, configuration files and more are located\nxdg-open is require for this command to function correctly";

pub fn main() {
	let dir_location = setup::get_applejuice_dir();
	status!("Opening data folder at '{}'...", dir_location);
	
	process::Command::new("xdg-open")
		.arg(dir_location)
		.spawn()
		.expect("Failed to open data folder, is xdg-open installed?");
}
