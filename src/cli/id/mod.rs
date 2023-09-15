mod classic;

use anyhow::Result;
use clap::{Args, Subcommand};

use self::classic::{id_classic_command, TnnIdClassicArgs};

#[derive(Args)]
pub struct TnnIdCli {
	#[command(subcommand)]
	pub command: TnnIdCommands,
}

#[derive(Subcommand)]
pub enum TnnIdCommands {
	Classic(TnnIdClassicArgs),
}

pub async fn id_command(args: &TnnIdCli) -> Result<()> {
	match &args.command {
		TnnIdCommands::Classic(args) => id_classic_command(args).await?,
	}

	Ok(())
}
