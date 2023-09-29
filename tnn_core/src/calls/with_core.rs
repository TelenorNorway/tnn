use ext_tnn::{call, Call};

use crate::Core;

pub struct WithCore(pub Box<dyn Fn(Core) -> Core>);

pub const WITH_CORE: Call<WithCore, ()> = call!(WithCore, (), "WITH_CORE", crate::NAME);