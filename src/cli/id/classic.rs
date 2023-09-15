use crate::{
	classic_id::{get_classic_id, TokenType},
	status::status_action,
};
use anyhow::Result;
use clap::Args;
use dialoguer::{theme::ColorfulTheme, Input};

#[derive(Args)]
pub struct TnnIdClassicArgs {
	/// Type of token to extract
	#[arg(short, long, default_value_t, value_enum)]
	pub r#type: TokenType,
}

pub async fn id_classic_command(_args: &TnnIdClassicArgs) -> Result<()> {
	get_classic_id(
		&get_origin_url_from_stdin,
		&get_origin_url_from_stdin,
		&get_origin_url_from_stdin,
		&get_origin_url_from_stdin,
	)
	.await?;

	Ok(())
}

async fn get_origin_url_from_stdin() -> Result<String> {
	match Input::<String>::with_theme(&ColorfulTheme::default())
		.with_prompt("Origin url")
		.interact_text()
	{
		Ok(value) => Ok(value),
		Err(err) => Err(err.into()),
	}
}
