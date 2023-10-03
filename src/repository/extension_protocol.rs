use anyhow::Result;
use std::{
	collections::{HashMap, HashSet},
	future::Future,
	pin::Pin,
	sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{
	extension::{
		opaque_fn::OpaqueFunctionCall,
		repository::{ExtensionAcessDeniedError, Protocol},
		CallNotFoundError,
	},
	util::State,
};

pub struct ExtensionProtocol {
	pub extension_name: &'static str,
	repo_extension_states: Arc<Mutex<HashMap<&'static str, Arc<Mutex<State>>>>>,
	extension_dependencies_resolved: Arc<Mutex<HashMap<&'static str, HashSet<&'static str>>>>,
	extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunctionCall>>>>,
}

impl ExtensionProtocol {
	pub fn new(
		extension_name: &'static str,
		repo_extension_states: Arc<Mutex<HashMap<&'static str, Arc<Mutex<State>>>>>,
		extension_dependencies_resolved: Arc<Mutex<HashMap<&'static str, HashSet<&'static str>>>>,
		extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunctionCall>>>>,
	) -> Self {
		Self {
			extension_name,
			repo_extension_states,
			extension_dependencies_resolved,
			extension_calls,
		}
	}

	pub async fn assert_access(&self, dependency: &'static str) -> Result<()> {
		if dependency == "" {
			return Ok(());
		} else if let Some(verified_dependencies) = self
			.extension_dependencies_resolved
			.lock()
			.await
			.get(self.extension_name)
		{
			if verified_dependencies.contains(dependency) {
				return Ok(());
			}
		}
		Err(ExtensionAcessDeniedError(self.extension_name, dependency).into())
	}
}

impl Protocol for ExtensionProtocol {
	fn get_dependency_state(
		&self,
		dependency: &'static str,
	) -> Pin<Box<dyn Future<Output = Result<Arc<Mutex<State>>>> + '_>> {
		Box::pin(async move {
			self.assert_access(dependency).await?;
			if let Some(state) = self.repo_extension_states.lock().await.get(dependency) {
				Ok(Arc::clone(state))
			} else {
				Err(ExtensionNotFound(dependency).into())
			}
		})
	}

	fn get_handler_for_call(
		&self,
		call_owner: &'static str,
		call_id: &'static str,
	) -> Pin<Box<dyn Future<Output = Result<Arc<OpaqueFunctionCall>>> + '_>> {
		Box::pin(async move {
			self.assert_access(call_owner).await?;
			if let Some(handler) = self.extension_calls.lock().await.get(call_id) {
				Ok(Arc::clone(handler))
			} else {
				Err(CallNotFoundError(self.extension_name, call_id).into())
			}
		})
	}
}

/// A given extension name cannot be found in the repository.
/// - The extension name
///
/// This should never happen! Panic.
#[derive(Error, Debug)]
#[error("Extension '{0}' not found!")]
struct ExtensionNotFound(&'static str);
