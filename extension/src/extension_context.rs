use anyhow::Result;
use std::{future::Future, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;
use util_tnn_state::State;

use crate::{
	call::Call,
	call_context::CallContext,
	repository::{AddCallArgs, Repository, ADD_CALL},
	CallOutput, Mixin,
};

#[repr(C)]
pub struct ExtensionContext {
	extension_name: &'static str,
	/// A flag for wheter this context is locked
	locked: Arc<Mutex<bool>>,
	repository: Repository,
	pub state: Arc<Mutex<State>>,
}

impl ExtensionContext {
	pub fn new(state: Arc<Mutex<State>>, extension_name: &'static str, repository: Repository) -> Self {
		Self {
			extension_name,
			locked: Arc::new(Mutex::new(false)),
			repository,
			state,
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

	pub async fn add_mixin<Payload: Sized, ReturnType: Future<Output = Result<()>>>(
		&self,
		_mixin: Mixin<Payload>,
		_subscription_approver: &'static impl Fn(CallContext<&'static str>) -> ReturnType,
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
		call: &'static Call<Argument, Return>,
		argument: Argument,
	) -> Result<Return> {
		let state = self.repository.0.get_dependency_state(call.owner).await?;

		unsafe {
			self.repository
				.0
				.get_handler_for_call(call.owner, call.id)
				.await?
				.invoke(CallContext::new(state, self.extension_name, argument))
		}
		.await
	}

	pub fn add_call<Argument: Sized, Return: Sized, Handler: Fn(CallContext<Argument>) -> CallOutput<Return>>(
		&self,
		call: &'static Call<Argument, Return>,
		handler: &'static Handler,
	) -> impl Future<Output = Result<()>> + '_ {
		let opaque = crate::opaque_fn::OpaqueFunction::from(handler);

		async {
			self.assert_not_locked().await?;

			self.call::<AddCallArgs, ()>(
				&ADD_CALL,
				AddCallArgs {
					call: call.id,
					handler: opaque,
				},
			)
			.await
		}
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

#[derive(Error, Debug)]
#[error("Extension '{0}' tried to call '{1}', but call handler could not be downcasted.")]
pub struct CouldNotGetCallHandler(&'static str, &'static str);
