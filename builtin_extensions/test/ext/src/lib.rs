use anyhow::Result;
use clap::Parser;
use tnn::{
	core::api::{add_command, NAME},
	extension::{Dependency, Extension, ExtensionContext},
};

#[no_mangle]
pub static MANIFEST: Extension = Extension {
	name: env!("CARGO_PKG_NAME"),
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[Dependency::Required(NAME, ">= 0")],
	init: &|ctx| Box::pin(async { init(ctx).await }),
};

async fn init(ctx: ExtensionContext) -> Result<()> {
	add_command(&ctx, &test_command).await?;
	tnn::debug!("Hello from test extension");
	Ok(())
}

async fn test_command(args: TestArgs) -> Result<()> {
	tnn::info!("Hello, {}", args.name);

	Ok(())
}

#[derive(Parser)]
#[command(name = "test", author, version)]
struct TestArgs {
	name: String,
}
