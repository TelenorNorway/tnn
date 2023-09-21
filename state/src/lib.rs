// Copyright 2023 Telenor. All rights reserved. MIT license.

// Forbed from crates.io:
// https://docs.rs/gotham_state/1.0.1/src/gotham_state/lib.rs.html
// Copyright 2022 James Bradlee. All rights reserved. MIT license.

// Forked from Deno:
// https://github.com/denoland/deno/blob/1fb5858009f598ce3f917f9f49c466db81f4d9b0/core/gotham_state.rs
// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

// Forked from Gotham:
// https://github.com/gotham-rs/gotham/blob/bcbbf8923789e341b7a0e62c59909428ca4e22e2/gotham/src/state/mod.rs
// Copyright 2017 Gotham Project Developers. MIT license.

use std::any::type_name;
use std::any::Any;
use std::any::TypeId;
use std::collections::BTreeMap;

use anyhow::Result;
use thiserror::Error;

#[derive(Default, Debug)]
pub struct State {
	data: BTreeMap<TypeId, Box<dyn Any>>,
}

impl State {
	/// Puts a value into the `State` storage. One value of each type is retained.
	/// Successive calls to `put` will overwrite the existing value of the same
	/// type.
	pub fn put<T: 'static>(&mut self, t: T) {
		let type_id = TypeId::of::<T>();
		self.data.insert(type_id, Box::new(t));
	}

	/// Determines if the current value exists in `State` storage.
	pub fn has<T: 'static>(&self) -> bool {
		let type_id = TypeId::of::<T>();
		self.data.get(&type_id).is_some()
	}

	/// Tries to borrow a value from the `State` storage.
	pub fn try_borrow<T: 'static>(&self) -> Option<&T> {
		let type_id = TypeId::of::<T>();
		self.data.get(&type_id).and_then(|b| b.downcast_ref())
	}

	/// Borrows a value from the `State` storage.
	pub fn borrow<T: 'static>(&self) -> Result<&T> {
		if let Some(value) = self.try_borrow() {
			Ok(value)
		} else {
			Err(MissingTypeError::from::<T>().into())
		}
	}

	/// Tries to mutably borrow a value from the `State` storage.
	pub fn try_borrow_mut<T: 'static>(&mut self) -> Option<&mut T> {
		let type_id = TypeId::of::<T>();
		self.data.get_mut(&type_id).and_then(|b| b.downcast_mut())
	}

	/// Mutably borrows a value from the `State` storage.
	pub fn borrow_mut<T: 'static>(&mut self) -> Result<&mut T> {
		if let Some(value) = self.try_borrow_mut() {
			Ok(value)
		} else {
			Err(MissingTypeError::from::<T>().into())
		}
	}

	/// Tries to move a value out of the `State` storage and return ownership.
	pub fn try_take<T: 'static>(&mut self) -> Option<T> {
		let type_id = TypeId::of::<T>();
		self.data
			.remove(&type_id)
			.and_then(|b| b.downcast().ok())
			.map(|b| *b)
	}

	/// Moves a value out of the `State` storage and returns ownership.
	pub fn take<T: 'static>(&mut self) -> Result<T> {
		if let Some(value) = self.try_take() {
			Ok(value)
		} else {
			Err(MissingTypeError::from::<T>().into())
		}
	}
}

#[derive(Error, Debug)]
#[error("required type {0} is not present in State container")]
pub struct MissingTypeError(&'static str);

impl MissingTypeError {
	fn from<T: 'static>() -> MissingTypeError {
		MissingTypeError(type_name::<T>())
	}
}
