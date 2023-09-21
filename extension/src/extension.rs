use anyhow::Result;
use std::{future::Future, pin::Pin, sync::Arc};

use crate::{Dependency, ExtensionContext};

#[repr(C)]
pub struct Extension {
	pub name: &'static str,
	pub version: &'static str,
	pub dependencies: &'static [Dependency],
	pub init: &'static (dyn (Fn(Arc<ExtensionContext>) -> Pin<Box<dyn Future<Output = Result<()>>>>) + Sync),
}
