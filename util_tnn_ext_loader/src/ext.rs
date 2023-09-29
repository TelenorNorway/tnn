use core::panic;

use crate::os::{os_type, OperativeSystem};

static WINDOWS: &'static str = "dll";
static MACOS: &'static str = "dylib";
static LINUX: &'static str = "so";

pub fn ext() -> &'static str {
	match os_type() {
		OperativeSystem::Windows => WINDOWS,
		OperativeSystem::Linux => LINUX,
		OperativeSystem::Darwin => MACOS,
		OperativeSystem::Unknown => panic!("Operative system not supported!"),
	}
}
