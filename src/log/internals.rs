pub use std::format_args;

#[derive(PartialEq)]
pub enum Level {
	Debug,
	Info,
	Warning,
	Critical,
}

static DEBUG_STYLES: &'static str = "\u{001b}[1;90m       Debug\u{001b}[0m";
static INFO_STYLES: &'static str = "\u{001b}[1;94m        Info\u{001b}[0m";
static WARNING_STYLES: &'static str = "\u{001b}[1;93m     Warning\u{001b}[0m";
static CRITICAL_STYLES: &'static str = "\u{001b}[1;91m    Critical\u{001b}[0m";

fn level_prefix(level: Level) -> &'static str {
	match level {
		Level::Debug => DEBUG_STYLES,
		Level::Info => INFO_STYLES,
		Level::Warning => WARNING_STYLES,
		Level::Critical => CRITICAL_STYLES,
	}
}

pub fn log(level: Level, message: String) {
	#[cfg(not(debug_assertions))]
	if std::env::var_os("TNN_DEBUG").is_none() {
		return;
	}
	println!("{} {}", level_prefix(level), message);
}
