use std::cell::Cell;

pub struct LogContext(());

thread_local! {
	pub static _INDENTATION: Cell<usize> = Cell::new(0);
}

impl LogContext {
	pub fn set_indentation(indentation: usize) {
		_INDENTATION.with(|cell| cell.set(indentation));
	}

	pub fn get_indentation() -> usize {
		_INDENTATION.with(|cell| cell.get())
	}

	pub fn _new() -> Self {
		_INDENTATION.with(|indentation| indentation.set(indentation.get() + 1));
		LogContext(())
	}
}

impl Drop for LogContext {
	fn drop(&mut self) {
		_INDENTATION.with(|indentation| indentation.set(indentation.get() - 1));
	}
}

macro_rules! _log {
	($type:literal, $format:literal $(, $($argument:expr),*)?) => {
		{
			use std::{cell::Cell, io::{Write, stderr}};

			let indentation = $crate::utils::terminal::_INDENTATION.with(Cell::get);
			let prefix = "\t".repeat(indentation);

			let mut output = stderr().lock();
			if let Err(error) = write!(output, "{prefix}{}", $type)
					.and_then(|()| writeln!(output, $format $(, $($argument),*)?)) {
				panic!("failed printing to stderr: {error}");
			}

			$crate::utils::terminal::LogContext::_new()
		}
	}
}

macro_rules! status {
	($($token:tt)*) => {$crate::utils::terminal::_log!("\x1b[94m\x1b[1m• \x1b[0m", $($token)*)}
}

macro_rules! help {
	($($token:tt)*) => {$crate::utils::terminal::_log!("\x1b[92m\x1b[1m? \x1b[0m", $($token)*)}
}

macro_rules! success {
	($($token:tt)*) => {$crate::utils::terminal::_log!("\x1b[92m\x1b[1m✓ \x1b[0m", $($token)*)}
}

macro_rules! warning {
	($($token:tt)*) => {$crate::utils::terminal::_log!("\x1b[93m\x1b[1m! \x1b[0m", $($token)*)}
}

macro_rules! error {
	($($token:tt)*) => {$crate::utils::terminal::_log!("\x1b[91m\x1b[1mX \x1b[0m", $($token)*)}
}

pub(crate) use {error, help, status, success, warning, _log};
