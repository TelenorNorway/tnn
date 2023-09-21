use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use ext::ext;
use extension::Extension;
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

	logs::debug!("Scanning {:?}", pattern);
	for result in glob(pattern.to_str().unwrap()).expect("") {
		match result {
			Err(err) => logs::critical!("{:?}", err),
			Ok(path) => {
				logs::debug!("Loading {:?}", path);
				match load_from_library(path.to_str().unwrap()) {
					Ok(extension) => {
						logs::debug!("Loaded [{}] {:?}", extension.name, path);
						extensions.push(extension);
					}
					Err(error) => {
						logs::warn!("Could not load extension from {:?}: {}", path, error);
					}
				};
			}
		}
	}

	Ok(extensions)
}
