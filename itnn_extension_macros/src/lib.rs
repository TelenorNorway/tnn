use proc_macro::TokenStream;

mod extension;
mod op;

/// A helper macro to define an operation.
///
/// This macro will transform your function into an acceptable
#[proc_macro_attribute]
pub fn op(args: TokenStream, input: TokenStream) -> TokenStream {
	let _args: TokenStream = args.into();
	op::expand(syn::parse2::<syn::Item>(input.into()).expect("could not get syntax tree of item")).into()
}

#[proc_macro]
pub fn extension(input: TokenStream) -> TokenStream {
	match extension::parse(input.into()) {
		Err(token_stream) => token_stream.into(),
		Ok(ext) => extension::expand(ext).into(),
	}
}
