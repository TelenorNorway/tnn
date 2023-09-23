use ext_tnn::{repository::AddCallArgs, CallContext, CallOutput};

pub fn add_call(ctx: CallContext<AddCallArgs>) -> CallOutput<()> {
	Box::pin(async move {
		util_tnn_logs::debug!("Extension {} wants to implement call {}", ctx.caller, ctx.argument.call);
		Ok(())
	})
}
