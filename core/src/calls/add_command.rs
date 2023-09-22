use anyhow::Result;
use clap::Arg;
use ext_tnn::{call, Call};
use std::{future::Future, pin::Pin};

pub struct AddCommand<'a>(pub Arg, pub &'a fn() -> Pin<Box<dyn Future<Output = Result<()>>>>);

pub const ADD_COMMAND: Call<AddCommand<'static>, ()> = call!(AddCommand<'static>, (), "ADD_COMMAND", crate::NAME);
