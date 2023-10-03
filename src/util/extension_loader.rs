use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use glob::glob;
use libloading::{Library, Symbol};

use crate::extension::Extension;

#[cfg(windows)]
const FILE_EXTENSION: &'static str = "dll";
#[cfg(linux)]
const FILE_EXTENSION: &'static str = "so";
#[cfg(target_os = "macos")]
const FILE_EXTENSION: &'static str = "dylib";

pub fn load_from_library(path: &str) -> Result<&'static Extension> {
	unsafe {
		let lib = Library::new(path)?;
		let manifest: Symbol<&'static Extension> = lib.get(b"MANIFEST")?;
		Ok(*manifest)
	}
}

pub fn load_from_directory(directory: &PathBuf) -> Result<Vec<&'static Extension>> {
	let pattern = directory.join(PathBuf::from_str(format!("*.{}", FILE_EXTENSION).as_str())?);

	let mut extensions: Vec<&'static Extension> = Vec::new();

	crate::debug!("Scanning {:?}", pattern);
	for result in glob(pattern.to_str().unwrap()).expect("") {
		match result {
			Err(err) => crate::critical!("{:?}", err),
			Ok(path) => {
				crate::debug!("Loading {:?}", path);
				match load_from_library(path.to_str().unwrap()) {
					Ok(extension) => {
						crate::debug!("Loaded [{}] {:?}", extension.name, path);
						extensions.push(extension);
					}
					Err(error) => {
						crate::warn!("Could not load extension from {:?}: {}", path, error);
					}
				};
			}
		}
	}

	Ok(extensions)
}
