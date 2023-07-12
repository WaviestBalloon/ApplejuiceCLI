use crate::utils::setup;

pub fn main() {
	println!("Checking for existing configuration directory...");
	if setup::confirm_applejuice_data_folder_existence() {
		println!("Configuration directory already exists!");
	} else {
		println!("Constructing configuration directory...");
		setup::construct_applejuice_data_folder();
	}

	println!("\nApplejuice has been initialised!\nTo get started, run 'applejuicecli --help'\nOr to dive right in, run 'applejuicecli --install client'");
}
