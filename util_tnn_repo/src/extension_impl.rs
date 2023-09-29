use crate::repository_context::RepositoryContext;
use ext_tnn::{repository::AddCallArgs, CallContext, CallOutput};
use std::sync::Arc;

pub fn add_call(ctx: CallContext<AddCallArgs>) -> CallOutput<()> {
	Box::pin(async move {
		util_tnn_logs::debug!("Extension {} wants to implement call {}", ctx.caller, ctx.argument.call);
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
		util_tnn_logs::debug!("Extension {} added call {}", ctx.caller, ctx.argument.call);
		Ok(())
	})
}
