use std::sync::Arc;

use tokio::sync::Mutex;
use util_tnn_state::State;

pub struct CallContext<Argument: Sized> {
	pub state: Arc<Mutex<State>>,
	pub caller: &'static str,
	pub data: Argument,
}
