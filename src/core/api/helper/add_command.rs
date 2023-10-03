use std::future::Future;

use crate::{core::api::calls, extension::ExtensionContext};
use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};

pub async fn add_command<Command: CommandFactory + FromArgMatches + 'static, Output: Future<Output = Result<()>>>(
	ctx: &ExtensionContext,
	command: &'static impl Fn(Command) -> Output,
) -> Result<()> {
	ctx.call(
		&calls::WITH_CORE,
		calls::WithCore(Box::new(move |core| {
			core.add_command::<Command>(Box::new(move |args| Box::pin(async move { command(args).await })))
		})),
	)
	.await
}
