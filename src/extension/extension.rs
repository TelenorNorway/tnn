use super::{CallOutput, Dependency, ExtensionContext};

#[repr(C)]
pub struct Extension {
	pub name: &'static str,
	pub version: &'static str,
	pub dependencies: &'static [Dependency],
	pub init: &'static (dyn (Fn(ExtensionContext) -> CallOutput<()>) + Sync),
}
