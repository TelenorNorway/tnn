#[tnn::op]
pub fn greet(input: String) -> String {
	println!("Called by {}, with input = {}", tnn_operation_context.caller, input);
	format!("Hello, {}!", input)
}
