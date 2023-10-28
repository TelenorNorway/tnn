use std::marker::PhantomData;

use serde::Deserialize;

pub struct Operation<'a, Input: Deserialize<'a>, Output: serde::Serialize> {
	pub name: &'static str,
	pub owner: &'static str,
	pub id: &'static str,
	pub _phantom: PhantomData<(&'a (), Input, Output)>,
}

pub struct OpContext<'a, Input: Deserialize<'a>> {
	pub input: Input,
	pub caller: &'static str,
	_phantom: PhantomData<&'a ()>,
}

impl<'a, Input: Deserialize<'a>> OpContext<'a, Input> {
	pub fn new(input: Input, caller: &'static str) -> Self {
		Self {
			_phantom: PhantomData,
			input,
			caller,
		}
	}
}
