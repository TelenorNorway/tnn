#[tnn::op]
pub fn hello() -> String {
	println!("Called by {}", tnn_operation_context.caller);
	format!("Hello, World!")
}
