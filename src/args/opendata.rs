use crate::utils::{setup, terminal::*};
use std::process;

pub fn main() {
	let dir_location = setup::get_applejuice_dir();
	status!("Opening data folder at '{}'...", dir_location);
	
	let _ = process::Command::new("xdg-open")
		.arg(dir_location)
		.spawn()
		.expect("Failed to open data folder, is xdg-open installed?").wait();
}
