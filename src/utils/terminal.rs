use std::process;

pub fn error<S: AsRef<str>>(message: S) {
	println!("\x1b[91m\x1b[1m[ x ]\x1b[0m \x1b[91m{}\x1b[0m", message.as_ref());
	process::exit(1);
}

pub fn warning<S: AsRef<str>>(message: S) {
	println!("\x1b[93m\x1b[1m[ ! ]\x1b[0m {}", message.as_ref());
}

pub fn success<S: AsRef<str>>(message: S) {
	println!("\x1b[92m\x1b[1m[ ✓ ]\x1b[0m {}", message.as_ref());
}

pub fn status<S: AsRef<str>>(message: S) {
	println!("\x1b[94m\x1b[1m      •\x1b[0m {}", message.as_ref());
}

pub fn statusdownload<S: AsRef<str>>(message: S) {
	print!("\x1b[94m\x1b[1m      •\x1b[0m {}", message.as_ref());
}
pub fn statusprogress<S: AsRef<str>>(message: S) {
	println!("\x1b[94m\x1b[1m       🡲 {}\x1b[0m", message.as_ref());
}
