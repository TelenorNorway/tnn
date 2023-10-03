use crate::{
	call,
	core::api::{calls::WithCore, calls::WITH_CORE, Core, NAME},
	extension::{Call, CallContext, CallOutput, Extension, ExtensionContext},
	util::parent_command::ParentCommand,
};
use anyhow::Result;
use clap::CommandFactory;
use thiserror::Error;

use super::app::App;

pub const RUN: Call<(), ()> = call!((), (), "run", NAME);

pub static MANIFEST: Extension = Extension {
	name: env!("CARGO_PKG_NAME"),
	version: env!("CARGO_PKG_VERSION"),
	dependencies: &[],
	init: &|ctx| Box::pin(async { core_init(ctx).await }),
};

async fn core_init(ctx: ExtensionContext) -> Result<()> {
	ctx.state
		.lock()
		.await
		.put(Core::new(ParentCommand::new(<App as CommandFactory>::command())));
	crate::debug!("Added core");

	ctx.add_call(&RUN, &run).await?;
	ctx.add_call(&WITH_CORE, &with_core).await?;
	Ok(())
}

fn with_core(ctx: CallContext<WithCore>) -> CallOutput<()> {
	Box::pin(async move {
		let mut state = ctx.state.lock().await;
		crate::debug!("Taking core by {}", ctx.caller);
		let core1 = state.take::<Core>()?;
		crate::debug!("Core stolen by {}", ctx.caller);
		state.put(ctx.argument.0(core1)?);
		crate::debug!("Core given back by {}", ctx.caller);
		Ok(())
	})
}

fn run(ctx: CallContext<()>) -> CallOutput<()> {
	Box::pin(async move {
		if ctx.caller != "" {
			return Err(CallerNotAllowedError("tnn/run", ctx.caller).into());
		}

		let (command, handler) = ctx.state.lock().await.take::<Core>()?.finish().build();

		let matches = command.get_matches();

		crate::debug!("Running application!");
		handler(&matches).await?;

		Ok(())
	})
}

#[derive(Error, Debug)]
#[error("Call '{0}' can only be executed by owner, called by '{1}'")]
struct CallerNotAllowedError(&'static str, &'static str);
