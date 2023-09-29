use anyhow::Result;
use clap::Parser;
use ext_tnn::{Dependency, Extension, ExtensionContext};
use tnn_core::add_command;

#[no_mangle]
pub static MANIFEST: Extension = Extension {
	name: env!("CARGO_PKG_NAME"),
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[Dependency::Required("core", ">= 0")],
	init: &|ctx| Box::pin(async { init(ctx).await }),
};

async fn init(ctx: ExtensionContext) -> Result<()> {
	add_command(&ctx, &test_command).await?;
	util_tnn_logs::debug!("Hello from test extension");
	Ok(())
}

async fn test_command(args: TestArgs) -> Result<()> {
	util_tnn_logs::info!("Hello, {}", args.name);

	Ok(())
}

#[derive(Parser)]
#[command(name = "test", author, version)]
struct TestArgs {
	name: String,
}
