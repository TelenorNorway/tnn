use std::sync::Arc;

use anyhow::Result;
use extension::{Extension, ExtensionContext};

pub static MANIFEST: Extension = Extension {
	name: "core",
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[],
	init: &|ctx| Box::pin(async { core_init(ctx).await }),
};

async fn core_init(_ctx: Arc<ExtensionContext>) -> Result<()> {
	logs::info!("Hello from Core extension's init()");
	Ok(())
}
