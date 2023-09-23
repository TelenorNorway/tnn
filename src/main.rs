use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use util_tnn_ext_loader::load_from_directory;
use util_tnn_repo::ExtensionRepository;

mod core;

#[tokio::main]
async fn main() -> Result<()> {
	// Create an extension repository. This lets extensions
	// interact with one another.
	let repository = ExtensionRepository::new().await;

	util_tnn_logs::debug!("Adding core extension!");
	// Injects the core extension.
	repository.add(&core::MANIFEST).await?;

	util_tnn_logs::debug!("Adding extensions from debug!");
	for extension in load_from_directory(&PathBuf::from_str("./target/debug")?)? {
		repository.add(extension).await?;
	}

	// Lock repository, no new extensions can be added throughout the lifetime
	// of the application.
	// repository.lock().await?;

	repository.print_problems().await;

	repository.call(&core::RUN, ()).await?;

	Ok(())
}
