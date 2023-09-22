use anyhow::Result;
use std::{any::Any, collections::HashMap, future::Future, pin::Pin, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;
use util_tnn_state::State;

use crate::{call::Call, call_context::CallContext, extension_communication::ExtensionCommunication, Mixin};

#[repr(C)]
pub struct ExtensionContext {
	extension_name: &'static str,
	/// A flag for wheter this context is locked
	locked: Arc<Mutex<bool>>,
	pub state: Arc<Mutex<State>>,
}

impl ExtensionContext {
	pub fn new(extension_name: &'static str) -> ExtensionContext {
		Self {
			extension_name,
			locked: Arc::new(Mutex::new(false)),
			state: Arc::new(Mutex::new(State::default())),
		}
	}

	async fn _lock(&self) {
		let mut locked = self.locked.lock().await;
		*locked = true;
	}

	pub async fn is_locked(&self) -> bool {
		*self.locked.lock().await
	}

	async fn assert_not_locked(&self) -> Result<()> {
		if self.is_locked().await {
			Err(LockedError.into())
		} else {
			Ok(())
		}
	}

	pub async fn add_call<Argument: Sized, Return: Sized, ReturnType: Future<Output = Result<Return>>>(
		&self,
		call: &'static Call<Argument, Return>,
		handler: &'static impl Fn(CallContext<Argument>) -> ReturnType,
	) -> Result<()> {
		self.assert_not_locked().await?;

		todo!("James Bradlee: implement this")
	}

	pub async fn add_mixin<Payload: Sized, ReturnType: Future<Output = Result<()>>>(
		&self,
		mixin: Mixin<Payload>,
		subscription_approver: &'static impl Fn(CallContext<&'static str>) -> ReturnType,
	) -> Result<()> {
		self.assert_not_locked().await?;

		todo!("James Bradlee: implement this")
	}

	pub async fn emit<Payload: Sized>(&self, _mixin: Mixin<Payload>, _payload: Payload) -> Result<()> {
		todo!("James Bradlee: implement this")
	}

	pub async fn emit_some<Payload: Sized>(
		_mixin: Mixin<Payload>,
		_payload: Payload,
		_receivers: &Vec<&'static str>,
	) -> Result<()> {
		todo!("James Bradlee: implement this")
	}

	pub async fn call<Argument: Sized, Return: Sized>(
		&self,
		_call: Call<Argument, Return>,
		_argument: Argument,
	) -> Result<Return> {
		todo!("James Bradlee: implement this")
	}
}

#[derive(Error, Debug)]
#[error("Extension '{0}' has already registered a handler for call '{0}/{1}'")]
pub struct DuplicateCallError(&'static str, &'static str);

#[derive(Error, Debug)]
#[error("Extension '{0}' tried to add handler for call '{1}/{2}'")]
pub struct NotCallOwnerError(&'static str, &'static str, &'static str);

#[derive(Error, Debug)]
#[error("Extension '{0}' has already registered an approver for mixin '{0}/{1}'")]
pub struct DuplicateMixinError(&'static str, &'static str);

#[derive(Error, Debug)]
#[error("Extension '{0}' tried to add approver for mixin '{1}/{2}'")]
pub struct NotMixinOwnerError(&'static str, &'static str, &'static str);

#[derive(Error, Debug)]
#[error("Context is locked and cannot be modified!")]
pub struct LockedError;
