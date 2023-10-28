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

/// A helper macro to define an extension.
///
/// Properties:
/// - `name`? ([str]) - Optionally define a custom name for your extension, defaults to the package name.
/// - `version`? ([str]) - Optionally define a custom version for your extension, defaults to the package versions.
/// - `dependencies`? (map `<dependency name> = [optional] "<dependency version req>"`), defaults to no dependencies.
///     ```
///       tnn::extension!(
///         dependencies = {
///           tnn = "^0.1",
///           my_cool_extension = optional "1.2.3"
///         }
///       )
///     ```
/// - `ops`? (`[path::to::op_function]`), defaults to no operations.
#[proc_macro]
pub fn extension(input: TokenStream) -> TokenStream {
	match extension::parse(input.into()) {
		Err(token_stream) => token_stream.into(),
		Ok(ext) => extension::expand(ext).into(),
	}
}
