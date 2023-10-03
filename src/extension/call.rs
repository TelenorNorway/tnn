use std::{future::Future, marker::PhantomData, pin::Pin};

use anyhow::Result;
use thiserror::Error;

pub type CallOutput<T> = Pin<Box<dyn Future<Output = Result<T>>>>;

pub struct Call<Argument, Return> {
	pub name: &'static str,
	pub owner: &'static str,
	pub id: &'static str,
	pub _phantom: PhantomData<(Argument, Return)>,
}

#[macro_export]
macro_rules! call {
	($arg:ty, $ret:ty, $name:expr, $owner:expr) => {
		$crate::extension::Call::<$arg, $ret> {
			name: $name,
			owner: $owner,
			id: $crate::extension::internal::concatcp!($owner, "/", $name),
			_phantom: std::marker::PhantomData,
		}
	};
}

#[derive(Error, Debug)]
#[error("Extension '{0}' tried to call '{1}', not found")]
pub struct CallNotFoundError(pub &'static str, pub &'static str);
