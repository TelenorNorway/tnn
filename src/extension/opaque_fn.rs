use super::{CallContext, CallOutput};

pub struct OpaqueFunctionCall([usize; 2]);

impl OpaqueFunctionCall {
	pub fn from<Argument, Return>(handler: &'static dyn Fn(CallContext<Argument>) -> CallOutput<Return>) -> Self {
		assert_eq!(
			std::mem::size_of::<&dyn Fn(CallContext<Argument>) -> CallOutput<Return>>(),
			2 * std::mem::size_of::<usize>()
		);
		Self(unsafe { std::mem::transmute(handler) })
	}
	pub unsafe fn invoke<Argument, Return>(&self, context: CallContext<Argument>) -> CallOutput<Return> {
		let handler: &dyn Fn(CallContext<Argument>) -> CallOutput<Return> = std::mem::transmute(self.0);
		handler(context)
	}
}
