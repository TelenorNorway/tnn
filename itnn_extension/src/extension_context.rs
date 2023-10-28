use crate::{OpContext, Operation};
use anyhow::Result;
use std::{future::Future, pin::Pin};

pub struct ExtensionContext {}

impl ExtensionContext {
	pub async fn add_operation<'a, Input: serde::Deserialize<'a>, Output: serde::Serialize>(
		&self,
		_operation: &'static Operation<'a, Input, Output>,
		_handler: &'static dyn Fn(OpContext<'_, Input>) -> Pin<Box<dyn Future<Output = Output>>>,
	) -> Result<()> {
		Ok(())
	}
}
