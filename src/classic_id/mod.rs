use std::future::Future;

use anyhow::{Error, Result};
use clap::ValueEnum;
use reqwest::{header::HeaderValue, Body, Method, Request, RequestBuilder, Url};
use serde::Serialize;
use thiserror::Error;

use crate::requests::get_client;

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TokenType {
	#[default]
	Jwt,
	Saml,
	Saml64,
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct InputError<'a>(&'a str);

pub async fn get_classic_id<
	'a,
	AsyncOrigin: Future<Output = Result<String>> + Sized,
	AsyncUsername: Future<Output = Result<String>> + Sized,
	AsyncOtp: Future<Output = Result<String>> + Sized,
	AsyncPassword: Future<Output = Result<String>> + Sized,
>(
	get_origin: impl Fn(Option<Error>) -> AsyncOrigin,
	get_username: impl Fn(Option<Error>) -> AsyncUsername,
	get_otp: impl Fn(Option<Error>) -> AsyncOtp,
	get_password: impl Fn(Option<Error>) -> AsyncPassword,
) -> Result<()> {
	let _ = {
		let mut previous_error: Option<Error> = None;
		loop {
			let value = get_origin(previous_error).await?;
			let origin_url = match Url::parse(&value) {
				Err(error) => {
					previous_error = Some(error.into());
					continue;
				}
				Ok(url) => url,
			};
		}
	};

	println!("Origin = {}", get_origin().await?);

	Ok(())
}

#[derive(Error, Debug)]
enum FetchLocationError {
	#[error("Got status {0}, expected 3xx")]
	InvalidStatus(u16),

	#[error("Missing location header in response")]
	NoLocationHeader,
}

async fn fetch_location<T: Into<Body>>(
	referrer: Option<&str>,
	url: Url,
	method: Method,
	body: Option<(T, &str)>,
) -> Result<Url> {
	let client = get_client().await;

	let mut request = Request::new(method, url.clone());

	if let Some(value) = referrer {
		request
			.headers_mut()
			.append("referrer", HeaderValue::from_str(value)?);
	}

	if let Some((body_value, content_type)) = body {
		*request.body_mut() = Some(body_value.into());
		request
			.headers_mut()
			.append("content-type", HeaderValue::from_str(content_type)?);
	}

	let response = client.execute(request).await?;

	{
		let status = response.status().as_u16();
		if !(300..399).contains(&status) {
			return Err(FetchLocationError::InvalidStatus(status).into());
		}
	}

	if let Some(location) = response.headers().get("location") {
		Ok(url.join(&location.to_str()?)?)
	} else {
		Err(FetchLocationError::NoLocationHeader.into())
	}
}
