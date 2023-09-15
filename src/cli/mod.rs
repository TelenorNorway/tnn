mod az;
mod id;
mod op;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

use self::az::{az_command, TnnAzCli};
use self::id::{id_command, TnnIdCli};
use self::op::{op_command, TnnOpCli};

#[derive(Parser)]
#[command(
	author = env!("CARGO_PKG_AUTHORS").split(':').next().unwrap(),
	version = env!("CARGO_PKG_VERSION"),
	about = env!("CARGO_PKG_DESCRIPTION"),
	long_about = None,
	propagate_version = true,
)]
pub struct TnnCli {
	#[command(subcommand)]
	pub command: TnnCommands,
}

#[derive(Subcommand)]
pub enum TnnCommands {
	Az(TnnAzCli),
	Op(TnnOpCli),
	Id(TnnIdCli),
}

pub async fn tnn_main() -> Result<()> {
	main_command(&TnnCli::parse()).await?;

	Ok(())
}

pub async fn main_command(args: &TnnCli) -> Result<()> {
	match &args.command {
		TnnCommands::Az(args) => az_command(args).await?,
		TnnCommands::Op(args) => op_command(args).await?,
		TnnCommands::Id(args) => id_command(args).await?,
	}

	Ok(())
}
