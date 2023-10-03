pub const NAME: &'static str = "core";
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod calls;

mod core;
pub use core::*;

mod helper;
pub use helper::*;
