use std::any::Any;

use crate::{call, mixin, Call, Extension, Mixin};
use anyhow::Result;

pub const ADD_EXTENSION: Call<&'static Extension, Result<()>> =
	call!(&'static Extension, Result<()>, "ADD_EXTENSION", "");

pub struct AddCallArgs {
	pub call: &'static dyn Any,
	pub handler: &'static dyn Any,
}

pub struct AddMixinArgs {
	pub mixin: &'static dyn Any,
	pub handler: &'static dyn Any,
}

pub const ADD_CALL: Call<AddCallArgs, Result<()>> = call!(AddCallArgs, Result<()>, "ADD_CALL", "");

pub const ADD_MIXIN: Call<AddMixinArgs, Result<()>> = call!(AddMixinArgs, Result<()>, "ADD_MIXIN", "");

pub const DEPENDENT_ACTIVATED: Mixin<&'static str> = mixin!(&'static str, "DEPENDENT_ACTIVATED", "");

/// Fired when the repository has been locked and
/// extensions can no longer be added.
pub const REPOSITORY_LOCKED: Mixin<()> = mixin!((), "REPOSITORY_LOCKED", "");
