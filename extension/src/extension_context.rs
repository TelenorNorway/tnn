use anyhow::Result;
use state::State;
use std::{any::Any, collections::HashMap, future::Future, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{call::Call, call_context::CallContext, Mixin};

#[repr(C)]
pub struct ExtensionContext {
	extension_name: &'static str,
	/// HashMap<call_name, handle_call(CallContext<T>)>
	calls: Arc<Mutex<HashMap<&'static str, &'static dyn Any>>>,
	/// HashMap<mixin_name, on_before_subscriber_added(&'static str)>
	mixins: Arc<Mutex<HashMap<&'static str, &'static dyn Any>>>,
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
			calls: Arc::new(Mutex::new(HashMap::new())),
			mixins: Arc::new(Mutex::new(HashMap::new())),
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

		if call.owner != self.extension_name {
			return Err(NotCallOwnerError(self.extension_name, call.owner, call.name).into());
		}

		let mut calls = self.calls.lock().await;

		if calls.contains_key(call.name) {
			return Err(DuplicateCallError(self.extension_name, call.name).into());
		}

		calls.insert(call.name, handler as &dyn Any);

		Ok(())
	}

	pub async fn add_mixin<Payload: Sized, ReturnType: Future<Output = Result<()>>>(
		&self,
		mixin: Mixin<Payload>,
		subscription_approver: &'static impl Fn(CallContext<&'static str>) -> ReturnType,
	) -> Result<()> {
		self.assert_not_locked().await?;

		if mixin.owner != self.extension_name {
			return Err(NotMixinOwnerError(self.extension_name, mixin.owner, mixin.name).into());
		}

		let mut mixins = self.mixins.lock().await;

		if mixins.contains_key(mixin.name) {
			return Err(DuplicateMixinError(self.extension_name, mixin.name).into());
		}

		mixins.insert(mixin.name, subscription_approver as &dyn Any);

		Ok(())
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
