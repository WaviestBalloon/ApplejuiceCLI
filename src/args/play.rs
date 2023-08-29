use crate::utils::setup;
use crate::utils::terminal::*;
use std::process;

const HELP_TEXT: &str = "\nUsage: TODO";

pub fn main() {
	status("Launching Roblox...");
	let dir_location = setup::get_applejuice_dir();

	// TODO: Maybe make this only appear when prefixdata is first created?
	warning("Log in with another device; web-view is currently broken. https://www.roblox.com/crossdevicelogin/ConfirmCode");
	
	process::Command::new("xdg-open")
		.arg(dir_location)
		.spawn()
		.expect("Failed to launch Roblox Player using Proton");
}
