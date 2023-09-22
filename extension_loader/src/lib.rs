use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use ext::ext;
use ext_tnn::Extension;
use glob::glob;
use libloading::{Library, Symbol};

mod ext;
mod os;

pub fn load_from_library(path: &str) -> Result<&'static Extension> {
	unsafe {
		let lib = Library::new(path)?;
		let manifest: Symbol<&'static Extension> = lib.get(b"MANIFEST")?;
		Ok(*manifest)
	}
}

pub fn load_from_directory(directory: &PathBuf) -> Result<Vec<&'static Extension>> {
	let pattern = directory.join(PathBuf::from_str(format!("*.{}", ext()).as_str())?);

	let mut extensions: Vec<&'static Extension> = Vec::new();

	util_tnn_logs::debug!("Scanning {:?}", pattern);
	for result in glob(pattern.to_str().unwrap()).expect("") {
		match result {
			Err(err) => util_tnn_logs::critical!("{:?}", err),
			Ok(path) => {
				util_tnn_logs::debug!("Loading {:?}", path);
				match load_from_library(path.to_str().unwrap()) {
					Ok(extension) => {
						util_tnn_logs::debug!("Loaded [{}] {:?}", extension.name, path);
						extensions.push(extension);
					}
					Err(error) => {
						util_tnn_logs::warn!("Could not load extension from {:?}: {}", path, error);
					}
				};
			}
		}
	}

	Ok(extensions)
}
