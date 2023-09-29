use std::future::Future;

use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use ext_tnn::ExtensionContext;
pub use util_tnn_parent_command::CommandHandlerReturnType;

use crate::calls::WithCore;

pub async fn add_command<Command: CommandFactory + FromArgMatches + 'static, Output: Future<Output = Result<()>>>(
	ctx: &ExtensionContext,
	command: &'static impl Fn(Command) -> Output,
) -> Result<()> {
	ctx.call(
		&crate::calls::WITH_CORE,
		WithCore(Box::new(move |core| {
			core.add_command::<Command>(Box::new(move |args| Box::pin(async move { command(args).await })))
		})),
	)
	.await
}
