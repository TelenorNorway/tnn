use anyhow::Result;
use clap::Subcommand;
use ext_tnn::{Dependency, Extension, ExtensionContext};
use tnn_core::calls::WithCore;

#[no_mangle]
pub static MANIFEST: Extension = Extension {
	name: env!("CARGO_PKG_NAME"),
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[Dependency::Required("core", ">= 0")],
	init: &|ctx| Box::pin(async { init(ctx).await }),
};

async fn init(_ctx: ExtensionContext) -> Result<()> {
	_ctx.call(&tnn_core::calls::WITH_CORE, WithCore(&|core| core.add_command()))
		.await?;
	util_tnn_logs::debug!("Hello from marketplace");
	Ok(())
}

#[derive(Subcommand, Debug)]
enum App {}
