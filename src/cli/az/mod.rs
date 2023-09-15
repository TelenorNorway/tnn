mod login;

use anyhow::Result;
use clap::{Args, Subcommand};

use self::login::{az_login_command, TnnAzLoginArgs};

#[derive(Args)]
pub struct TnnAzCli {
	#[command(subcommand)]
	pub command: TnnAzCommands,
}

#[derive(Subcommand)]
pub enum TnnAzCommands {
	Login(TnnAzLoginArgs),
}

pub async fn az_command(args: &TnnAzCli) -> Result<()> {
	match &args.command {
		TnnAzCommands::Login(args) => az_login_command(args).await?,
	}

	Ok(())
}
