mod secrets;
mod shell;

use anyhow::Result;
use clap::{Args, Subcommand};

use self::{
	secrets::{op_secrets_command, TnnOpSecretsCli},
	shell::{op_shell_command, TnnOpShellArgs},
};

#[derive(Args)]
pub struct TnnOpCli {
	#[command(subcommand)]
	pub command: TnnOpCommands,
}

#[derive(Subcommand)]
pub enum TnnOpCommands {
	Shell(TnnOpShellArgs),
	Secrets(TnnOpSecretsCli),
}

pub async fn op_command(args: &TnnOpCli) -> Result<()> {
	match &args.command {
		TnnOpCommands::Shell(args) => op_shell_command(args).await?,
		TnnOpCommands::Secrets(args) => op_secrets_command(args).await?,
	}

	Ok(())
}
