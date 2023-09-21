pub struct CallContext<Argument: Sized> {
	pub caller: &'static str,
	pub data: Argument,
}
