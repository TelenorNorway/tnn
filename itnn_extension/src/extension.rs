use crate::{Dependency, ExtensionContext};
use anyhow::Result;
use std::{future::Future, pin::Pin};

pub struct Extension {
	pub name: &'static str,
	pub version: &'static str,
	pub dependencies: &'static [Dependency],
	pub init: Option<&'static (dyn (Fn(ExtensionContext) -> Pin<Box<dyn Future<Output = Result<()>>>>) + Sync)>,
}
