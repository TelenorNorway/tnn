mod unseal;

use anyhow::Result;
use clap::{Args, Subcommand};

use self::unseal::{op_secrets_unseal_command, TnnOpSecretsUnsealArgs};

#[derive(Args)]
pub struct TnnOpSecretsCli {
	#[command(subcommand)]
	pub command: TnnOpSecretsCommands,
}

#[derive(Subcommand)]
pub enum TnnOpSecretsCommands {
	Unseal(TnnOpSecretsUnsealArgs),
}

pub async fn op_secrets_command(args: &TnnOpSecretsCli) -> Result<()> {
	match &args.command {
		TnnOpSecretsCommands::Unseal(args) => op_secrets_unseal_command(args).await?,
	}

	Ok(())
}
