use std::{collections::HashMap, sync::Arc};

use ext_tnn::opaque_fn::OpaqueFunctionCall;
use tokio::sync::Mutex;

pub struct RepositoryContext {
	pub extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunctionCall>>>>,
}

impl RepositoryContext {
	pub fn new(extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunctionCall>>>>) -> Self {
		Self { extension_calls }
	}
}
