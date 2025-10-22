use chrono::Local;
use colored::Colorize;
use log::{trace, LevelFilter, Log, Metadata, Record};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LoggerError(pub String);
pub type LoggerResult<T> = Result<T, LoggerError>;

static mut LOGGER: Logger = Logger { file: None };

/// This function modify the logger's field
///
/// This function mutate global state!
pub fn refresh(level: LevelFilter, log_file: Option<PathBuf>) -> LoggerResult<()> {
	if let Some(file) = &log_file {
		let log_directory = file.parent().unwrap();
		if let Err(err) = create_dir_all(&log_directory) {
			let msg = format!("Failed to create {} directory: {err}", log_directory.display()).red();
			println!("{msg}");
			return Err(LoggerError(msg.to_string()));
		}

		if let Err(err) = OpenOptions::new()
			.create(true)
			.truncate(false)
			.write(true)
			.open(&file)
		{
			let msg = format!("Failed to create {}: {err}", &file.display()).red();
			println!("{msg}");
			return Err(LoggerError(msg.to_string()));
		}
	}
	log::set_max_level(level);
	#[allow(static_mut_refs)]
	unsafe {
		LOGGER.file = log_file;
	}
	Ok(())
}

/// This function binds the logger and refresh the settings
///
/// This function mutate global state!
pub fn bind_logger(level: LevelFilter, log_file: Option<PathBuf>) -> LoggerResult<()> {
	refresh(level, log_file)?;

	#[allow(static_mut_refs)]
	unsafe {
		if let Err(err) = log::set_logger(&LOGGER) {
			let msg = format!("Failed set logger: {err}").red();
			println!("{msg}");
			return Err(LoggerError(msg.to_string()));
		}
	}
	trace!("Logger bound!");
	Ok(())
}

struct Logger {
	file: Option<PathBuf>,
}

impl Log for Logger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let log_line = log(record);
		if let Some(file_path) = &self.file {
			let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
			file.write_all(format!("{log_line}\n").as_bytes())
				.expect("Could not write to the log file");
		} else {
			println!("{log_line}");
		}
	}

	fn flush(&self) {}
}

fn log(record: &Record) -> String {
	let local = Local::now();
	format!(
		"[{}] [{}] {} ({}{}): {}",
		local.format("%Y-%m-%d %H:%M:%S%.3f"),
		record.level(),
		record.target(),
		record.file().unwrap_or("<Unknown>"),
		if let Some(line) = record.line() {
			format!(":{line}")
		} else {
			String::from(":<Unknown>")
		},
		record.args()
	)
}

#[macro_export]
macro_rules! log_err {
    ($message:expr, $error:ty) => {{
		let msg = format!("{}", $message);
		log::error!("{}", msg);
		Err(<$error as From<String>>::from(msg))
    }};
}