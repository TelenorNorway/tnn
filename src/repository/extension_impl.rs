use std::sync::Arc;

use crate::{
	extension::{repository::AddCallArgs, CallContext, CallOutput},
	repository::repository_context::RepositoryContext,
};

pub fn add_call(ctx: CallContext<AddCallArgs>) -> CallOutput<()> {
	Box::pin(async move {
		crate::debug!("Extension {} wants to implement call {}", ctx.caller, ctx.argument.call);
		// todo(James Bradlee): Validate call name
		ctx.state
			.lock()
			.await
			.borrow_mut::<RepositoryContext>()
			.expect("should never happen")
			.extension_calls
			.lock()
			.await
			.insert(ctx.argument.call, Arc::new(ctx.argument.handler));
		crate::debug!("Extension {} added call {}", ctx.caller, ctx.argument.call);
		Ok(())
	})
}
