use anyhow::Result;

mod classic_id;
mod cli;
mod not_implemented_error;
mod requests;
mod status;

use cli::tnn_main;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<()> {
	tnn_main().await?;

	let url_google = Url::parse("https://google.com/foo")?;
	println!("{}", url_google);

	let url_join_relative = url_google.join("/bar")?;
	println!("{}", url_join_relative);

	let url_join_absolute = url_join_relative.join("https://example.com")?;
	println!("{}", url_join_absolute);

	Ok(())
}
