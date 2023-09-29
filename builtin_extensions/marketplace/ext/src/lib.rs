use anyhow::Result;
use clap::Parser;
use ext_tnn::{Dependency, Extension, ExtensionContext};
use tnn_core::{add_command, CommandHandlerReturnType};

#[no_mangle]
pub static MANIFEST: Extension = Extension {
	name: env!("CARGO_PKG_NAME"),
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[Dependency::Required("core", ">= 0")],
	init: &|ctx| Box::pin(async { init(ctx).await }),
};

async fn init(ctx: ExtensionContext) -> Result<()> {
	add_command::<App>(&ctx, &handle_command).await?;
	util_tnn_logs::debug!("Hello from marketplace");
	Ok(())
}

fn handle_command(_args: &App) -> CommandHandlerReturnType {
	Box::pin(async {
		util_tnn_logs::debug!("Yo from marketplace");

		Ok(())
	})
}

#[derive(Parser)]
#[command(name = "test", author, version)]
struct App;
