use std::{collections::HashMap, sync::Arc};

use ext_tnn::opaque_fn::OpaqueFunction;
use tokio::sync::Mutex;

pub struct RepositoryContext {
	pub extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunction>>>>,
}

impl RepositoryContext {
	pub fn new(extension_calls: Arc<Mutex<HashMap<&'static str, Arc<OpaqueFunction>>>>) -> Self {
		Self { extension_calls }
	}
}
