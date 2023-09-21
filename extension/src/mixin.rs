pub struct Mixin<Payload: Sized> {
	pub name: &'static str,
	pub owner: &'static str,
	pub id: &'static str,
	pub _fuck_rust1: Option<Payload>,
}

#[macro_export]
macro_rules! mixin {
	($payload:ty, $name:expr, $owner:expr) => {{
		Mixin::<$payload> {
			name: $name,
			owner: $owner,
			id: $crate::internal::concatcp!($owner, "/", $name),
			_fuck_rust1: None,
		}
	}};
}
