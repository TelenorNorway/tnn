pub enum OperativeSystem {
	Windows,
	Linux,
	Darwin,
	Unknown,
}

pub fn os_type() -> OperativeSystem {
	if cfg!(windows) {
		OperativeSystem::Windows
	} else if cfg!(target_os = "macos") {
		OperativeSystem::Darwin
	} else if cfg!(unix) {
		OperativeSystem::Linux
	} else {
		OperativeSystem::Unknown
	}
}
