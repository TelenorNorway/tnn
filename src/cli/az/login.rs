use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct TnnAzLoginArgs {
	#[arg()]
	name: String,
}

pub async fn az_login_command(args: &TnnAzLoginArgs) -> Result<()> {
	let name = &args.name;
	println!("Hello, {name}!");

	Ok(())
}
