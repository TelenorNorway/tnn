use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::extension::opaque_fn::OpaqueFunctionCall;

pub struct RepositoryContext {
	pub extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunctionCall>>>>,
}

impl RepositoryContext {
	pub fn new(extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunctionCall>>>>) -> Self {
		Self { extension_calls }
	}
}
