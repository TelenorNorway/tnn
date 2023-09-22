use std::sync::Arc;

use anyhow::Result;
use ext_tnn::{call, Call, CallContext, Extension, ExtensionContext};
use thiserror::Error;

pub const RUN: Call<(), ()> = call!((), (), "run", tnn_core::NAME);

pub static MANIFEST: Extension = Extension {
	name: tnn_core::NAME,
	version: tnn_core::VERSION,
	dependencies: &[],
	init: &|ctx| Box::pin(async { core_init(ctx).await }),
};

struct CoreState {}

impl CoreState {
	fn new() -> Self {
		Self {}
	}
}

async fn core_init(ctx: Arc<ExtensionContext>) -> Result<()> {
	ctx.state.lock().await.put(CoreState::new());
	ctx.add_call(&tnn_core::calls::ADD_COMMAND, &add_command).await?;
	ctx.add_call(&RUN, &run).await?;
	Ok(())
}

async fn add_command(_ctx: CallContext<tnn_core::calls::AddCommand<'_>>) -> Result<()> {
	Ok(())
}

async fn run(ctx: CallContext<()>) -> Result<()> {
	if ctx.caller != tnn_core::NAME {
		return Err(CallerNotAllowedError("core/run", ctx.caller).into());
	}

	Ok(())
}

#[derive(Error, Debug)]
#[error("Call '{0}' can only be executed by owner, called by '{1}'")]
struct CallerNotAllowedError(&'static str, &'static str);
