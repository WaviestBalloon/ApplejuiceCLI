use std::process;

pub fn error(message: String) {
	println!("\x1b[91m\x1b[1m[ x ]\x1b[0m \x1b[91m{}\x1b[0m", message);
	process::exit(1);
}

pub fn warning(message: String) {
	println!("\x1b[93m\x1b[1m[ ! ]\x1b[0m {}", message);
}
