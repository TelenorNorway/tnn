use anyhow::Result;
use thiserror::Error;
use tokio::sync::Mutex;
use util_tnn_state::State;

use crate::{call, mixin, opaque_fn::OpaqueFunction, Call, Extension, Mixin};
use std::{any::Any, future::Future, pin::Pin, sync::Arc};

pub const ADD_EXTENSION: Call<&'static Extension, ()> = call!(&'static Extension, (), "ADD_EXTENSION", "");

pub struct AddCallArgs {
	pub call: &'static str,
	pub handler: OpaqueFunction,
}

pub struct AddMixinArgs {
	pub mixin: &'static dyn Any,
	pub handler: &'static dyn Any,
}

pub const ADD_CALL: Call<AddCallArgs, ()> = call!(AddCallArgs, (), "ADD_CALL", "");

pub const ADD_MIXIN: Call<AddMixinArgs, ()> = call!(AddMixinArgs, (), "ADD_MIXIN", "");

pub const DEPENDENT_ACTIVATED: Mixin<&'static str> = mixin!(&'static str, "DEPENDENT_ACTIVATED", "");

/// Fired when the repository has been locked and
/// extensions can no longer be added.
pub const REPOSITORY_LOCKED: Mixin<()> = mixin!((), "REPOSITORY_LOCKED", "");

pub trait Protocol {
	fn get_dependency_state(
		&self,
		dependency: &'static str,
	) -> Pin<Box<dyn Future<Output = Result<Arc<Mutex<State>>>> + '_>>;

	fn get_handler_for_call(
		&self,
		call_owner: &'static str,
		call_id: &'static str,
	) -> Pin<Box<dyn Future<Output = Result<Arc<OpaqueFunction>>> + '_>>;
}

pub struct Repository(pub Pin<Box<dyn Protocol>>);

#[derive(Error, Debug)]
#[error("Extension '{0}' tried to use '{1}', access denied")]
pub struct ExtensionAcessDeniedError(pub &'static str, pub &'static str);
