use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use ext_tnn::ExtensionContext;
pub use util_tnn_parent_command::CommandHandlerReturnType;

use crate::calls::WithCore;

pub async fn add_command<Command: CommandFactory + FromArgMatches + 'static>(
	ctx: &ExtensionContext,
	handler: &'static impl Fn(&Command) -> CommandHandlerReturnType,
) -> Result<()> {
	ctx.call(
		&crate::calls::WITH_CORE,
		WithCore(Box::new(move |core| core.add_command::<Command>(handler))),
	)
	.await
}
