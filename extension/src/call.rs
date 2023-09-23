use std::{future::Future, pin::Pin};

use anyhow::Result;
use thiserror::Error;

pub type CallOutput<T> = Pin<Box<dyn Future<Output = Result<T>>>>;

pub struct Call<Argument: Sized, Return: Sized> {
	pub name: &'static str,
	pub owner: &'static str,
	pub id: &'static str,
	pub _fuck_rust1: Option<Argument>,
	pub _fuck_rust2: Option<Return>,
}

#[macro_export]
macro_rules! call {
	($arg:ty, $ret:ty, $name:expr, $owner:expr) => {{
		Call::<$arg, $ret> {
			name: $name,
			owner: $owner,
			id: $crate::internal::concatcp!($owner, "/", $name),
			_fuck_rust1: None,
			_fuck_rust2: None,
		}
	}};
}

#[derive(Error, Debug)]
#[error("Extension '{0}' tried to call '{1}', not found")]
pub struct CallNotFoundError(pub &'static str, pub &'static str);
