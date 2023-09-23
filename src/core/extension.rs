use anyhow::Result;
use clap::Command;
use ext_tnn::{call, Call, CallContext, CallOutput, Extension, ExtensionContext};
use thiserror::Error;
use tnn_core::{
	calls::{WithCore, WITH_CORE},
	Core,
};

pub const RUN: Call<(), ()> = call!((), (), "run", tnn_core::NAME);

pub static MANIFEST: Extension = Extension {
	name: tnn_core::NAME,
	version: tnn_core::VERSION,
	dependencies: &[],
	init: &|ctx| Box::pin(async { core_init(ctx).await }),
};

async fn core_init(ctx: ExtensionContext) -> Result<()> {
	ctx.state.lock().await.put(Core::new(
		Command::new(env!("CARGO_PKG_NAME")).about(env!("CARGO_PKG_DESCRIPTION")),
	));
	ctx.add_call(&RUN, &run).await?;
	ctx.add_call(&WITH_CORE, &with_core).await?;
	Ok(())
}

fn with_core(ctx: CallContext<WithCore<'static>>) -> CallOutput<()> {
	Box::pin(async move {
		let mut state = ctx.state.lock().await;
		let core1 = state.take::<Core>()?;
		state.put(ctx.argument.0(core1));
		Ok(())
	})
}

fn run(ctx: CallContext<()>) -> CallOutput<()> {
	Box::pin(async move {
		if ctx.caller != "" {
			return Err(CallerNotAllowedError("core/run", ctx.caller).into());
		}

		let command = ctx.state.lock().await.take::<Core>()?.finish();

		let _matches = command.get_matches();

		util_tnn_logs::debug!("Running application!");

		Ok(())
	})
}

#[derive(Error, Debug)]
#[error("Call '{0}' can only be executed by owner, called by '{1}'")]
struct CallerNotAllowedError(&'static str, &'static str);
