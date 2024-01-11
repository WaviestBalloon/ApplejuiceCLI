use crate::utils::terminal::*;
use notify_rust::Notification;

pub fn create_notification(icon: &str, expire_time: i32, title: &str, body: &str) {
	help!("{title}: {body}");
	let _ = Notification::new()
		.summary(title)
		.body(body)
		.icon(icon)
		.appname("Applejuice")
		.timeout(expire_time)
		.show();
}
