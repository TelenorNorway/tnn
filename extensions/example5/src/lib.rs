use std::sync::Arc;

use anyhow::Result;
use extension::{call, Call, Dependency, Extension, ExtensionContext};

const NAME: &'static str = env!("CARGO_PKG_NAME");

pub static CALL1: Call<u32, u32> = call!(u32, u32, "Name", NAME);

#[no_mangle]
pub static MANIFEST: Extension = Extension {
	name: NAME,
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[
		Dependency::Required("example4", ">= 0"),
		Dependency::Required("non-existant-extension", ">= 0"),
	],
	init: &|ctx| Box::pin(async { init(ctx).await }),
};

async fn init(_ctx: Arc<ExtensionContext>) -> Result<()> {
	logs::info!("Hello from example3's init()");
	Ok(())
}
