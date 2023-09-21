use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use extension_loader::load_from_directory;
use extension_repository::ExtensionRepository;

mod core;

#[tokio::main]
async fn main() -> Result<()> {
	// Create an extension repository. This lets extensions
	// interact with one another.
	let repository = ExtensionRepository::new();

	logs::debug!("Adding core extension!");
	// Injects the core extension.
	repository.add(&core::MANIFEST).await?;

	logs::debug!("Adding extensions from debug!");
	for extension in load_from_directory(&PathBuf::from_str("./target/debug")?)? {
		repository.add(extension).await?;
	}

	// Lock repository, no new extensions can be added throughout the lifetime
	// of the application.
	// repository.lock().await?;

	repository.print_problems().await;

	Ok(())
}
