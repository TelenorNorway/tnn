use crate::core::api::NAME;
use crate::extension::Call;
use anyhow::Result;

use crate::core::api::Core;

pub struct WithCore(pub Box<dyn Fn(Core) -> Result<Core>>);

pub const WITH_CORE: Call<WithCore, ()> = crate::call!(WithCore, (), "WITH_CORE", NAME);
