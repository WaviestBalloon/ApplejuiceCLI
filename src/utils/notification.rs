use std::process;
use crate::utils::terminal::*;

pub fn create_notification(icon: &str, expire_time: &str, title: &str, body: &str) {
	let output = process::Command::new("notify-send")
		.arg("--app-name=Applejuice")
		.arg(format!("--icon={}", icon))
		.arg("--urgency=normal")
		.arg(format!("--expire-time={}", expire_time))
		.arg(title)
		.arg(body)
		.output();

	match output {
		Ok(_) => { },
		Err(errmsg) => { // Do not quit or panic here, since it's a non-critical error
			warning!("Failed to create notification, raw: '{}'\nError: {}", icon, errmsg);
			
			if icon.is_empty() {
				warning!("Failed to create notification twice; stopping creation.");
				return;
			}

			if errmsg.to_string().contains("No such file or directory (os error 2)") { // Fallback to default/no icon if we detect a missing file error
				warning!("Assuming a asset was missing; falling back to no icon...");
				create_notification("", expire_time, title, body);
			}
		}
	}
}
