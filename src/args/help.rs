use crate::utils::{argparse, terminal::*};

static ACCEPTED_PARAMS: [(&str, &str); 6] = [
	("--help", "Displays this help message"),
	("--init", "Initalised Applejuice, use if you have deleted any files in the applications/data folder directory"),
	("--launch", "Launch a Roblox instance directly from your terminal"),
	("--install", "Install Roblox Player or Roblox Studio"),
	("--purge", "Deletes either deployment cache or uninstalls Roblox"),
	("--opendata", "Open the data folder for Applejuice, where installations, cache, configuration files and more are located")
];

pub fn main() {
	help!("ApplejuiceCLI - Bootstrapper and manager to get Roblox to run on Linux\n\tUsage: applejuicecli [command]\n\tAccepted commands: \n{}", argparse::generate_help(ACCEPTED_PARAMS.to_vec()));
}
