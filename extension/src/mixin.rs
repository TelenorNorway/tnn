use std::marker::PhantomData;

pub struct Mixin<Payload: Sized> {
	pub name: &'static str,
	pub owner: &'static str,
	pub id: &'static str,
	pub _phantom: PhantomData<Payload>,
}

#[macro_export]
macro_rules! mixin {
	($payload:ty, $name:expr, $owner:expr) => {{
		Mixin::<$payload> {
			name: $name,
			owner: $owner,
			id: $crate::internal::concatcp!($owner, "/", $name),
			_phantom: std::marker::PhantomData,
		}
	}};
}
