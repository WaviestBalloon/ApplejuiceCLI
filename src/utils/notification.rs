use std::process;

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
		Err(errmsg) => {
			println!("Failed to create notification, raw: '{}'\nError: {}", icon, errmsg);
		}
	}
}
