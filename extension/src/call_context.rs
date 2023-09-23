use std::sync::Arc;

use tokio::sync::Mutex;
use util_tnn_state::State;

pub struct CallContext<Argument> {
	pub state: Arc<Mutex<State>>,
	pub caller: &'static str,
	pub argument: Argument,
}

impl<Argument> CallContext<Argument> {
	pub fn new(state: Arc<Mutex<State>>, caller: &'static str, argument: Argument) -> Self {
		Self {
			state,
			caller,
			argument,
		}
	}
}
