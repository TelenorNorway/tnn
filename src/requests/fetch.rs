use anyhow::Result;
use reqwest::{Request, Response};

use super::client::get_client;

pub async fn _fetch(request: Request) -> Result<Response> {
	let client = get_client().await;
	println!("{:?}", request);
	Ok(client.execute(request).await?)
}
