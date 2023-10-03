pub struct OpaqueFunction([usize; 2]);

impl OpaqueFunction {
	pub fn from<Argument, Return>(handler: &'static dyn Fn(Argument) -> Return) -> Self {
		assert_eq!(
			std::mem::size_of::<&dyn Fn(Argument) -> Return>(),
			2 * std::mem::size_of::<usize>()
		);
		Self(unsafe { std::mem::transmute(handler) })
	}
	pub unsafe fn invoke<Argument, Return>(&self, context: Argument) -> Return {
		let handler: &dyn Fn(Argument) -> Return = std::mem::transmute(self.0);
		handler(context)
	}
}
