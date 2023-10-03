use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use tnn::repository::ExtensionRepository;

#[tokio::main]
async fn main() -> Result<()> {
	// Create an extension repository. This lets extensions
	// interact with one another.
	let repository = ExtensionRepository::new().await;

	tnn::debug!("Adding core extension!");
	// Injects the core extension.
	repository.add(&tnn::core::extension::MANIFEST).await?;

	#[cfg(debug_assertions)]
	{
		tnn::debug!("Adding extensions from debug!");
		for extension in tnn::util::extension_loader::load_from_directory(&PathBuf::from_str("./target/debug")?)? {
			repository.add(extension).await?;
		}
	}
	#[cfg(not(debug_assertions))]
	{
		tnn::debug!("Adding extensions from TNN_HOME!");
	}

	// Lock repository, no new extensions can be added throughout the lifetime
	// of the application.
	repository.lock().await?;

	repository.print_problems().await;

	repository.call(&tnn::core::extension::RUN, ()).await?;

	Ok(())
}
