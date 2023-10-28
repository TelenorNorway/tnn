#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
	punctuated::Punctuated,
	spanned::Spanned,
	token::{Brace, Paren, PathSep},
	Block, Error, Expr, ExprAsync, ExprCall, ExprPath, FnArg, GenericArgument, Item, ItemFn, Pat, Path, PathArguments,
	PathSegment, ReturnType, Stmt, Token, Type, TypeParamBound, TypeTuple, Visibility,
};

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
	tokens.extend(error.into_compile_error());
	tokens
}

pub(crate) fn expand(item: Item) -> TokenStream {
	match item {
		Item::Fn(it) => match expand_operation_function(it) {
			Ok(tokens) => tokens,
			Err(tokens) => tokens,
		},
		_ => token_stream_with_error(
			item.to_token_stream(),
			Error::new(item.span(), "op can only be used on a function"),
		),
	}
}

fn default_output_for_function(item: &ItemFn) -> Result<ReturnType, TokenStream> {
	match "-> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()>>>".parse::<TokenStream>() {
		Ok(tokens) => match syn::parse2::<ReturnType>(tokens) {
			Ok(it) => Ok(it),
			Err(_) => panic!("this should never happen"),
		},
		Err(err) => {
			return Err(token_stream_with_error(
				item.to_token_stream(),
				Error::new(item.sig.ident.span(), err),
			))
		}
	}
}

fn transform_output_for_function(item: &ItemFn, output: &Box<Type>) -> Result<Box<Type>, TokenStream> {
	let default_type = default_output_for_function(item)?;
	if let ReturnType::Type(_, mut typ) = default_type {
		if let Type::Path(path) = typ.as_mut() {
			if let PathArguments::AngleBracketed(generics) = &mut path.path.segments[2].arguments {
				if let GenericArgument::Type(typ) = &mut generics.args[0] {
					if let Type::Path(path) = typ {
						if let PathArguments::AngleBracketed(generics) = &mut path.path.segments[0].arguments {
							if let GenericArgument::Type(typ) = &mut generics.args[0] {
								if let Type::TraitObject(obj) = typ {
									if let TypeParamBound::Trait(bound) = &mut obj.bounds[0] {
										if let PathArguments::AngleBracketed(generics) =
											&mut bound.path.segments[2].arguments
										{
											if let GenericArgument::AssocType(assoc) = &mut generics.args[0] {
												assoc.ty = *output.clone();
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		return Ok(typ);
	}
	panic!("this should never happen")
}

fn expand_operation_function(mut item: ItemFn) -> Result<TokenStream, TokenStream> {
	if item.sig.unsafety.is_some() {
		return Err(token_stream_with_error(
			item.to_token_stream(),
			Error::new(item.sig.unsafety.span(), "op should not be unsafe (NO_UNSAFE_OP)"),
		));
	}

	if item.sig.inputs.len() > 1 {
		return Err(token_stream_with_error(
			item.to_token_stream(),
			Error::new(item.sig.inputs[1].span(), "op should only accept a single parameter"),
		));
	}

	let input_info = if item.sig.inputs.len() < 1 {
		None
	} else {
		let (input_type, input_name) = match (&item.sig.inputs[0]).clone() {
			FnArg::Receiver(_) => {
				return Err(token_stream_with_error(
					item.to_token_stream(),
					Error::new(item.sig.inputs[0].span(), "parameter should be typed, not a receiver"),
				))
			}
			FnArg::Typed(typ) => (*typ.ty, {
				let Pat::Ident(tmp) = typ.pat.as_ref() else {
					return Err(token_stream_with_error(
						item.to_token_stream(),
						Error::new(typ.pat.span(), "parameter should be a binding identifier"),
					));
				};
				if tmp.by_ref.is_some() {
					return Err(token_stream_with_error(
						item.to_token_stream(),
						Error::new(tmp.by_ref.span(), "parameter must be owned"),
					));
				}
				if tmp.mutability.is_some() {
					return Err(token_stream_with_error(
						item.to_token_stream(),
						Error::new(tmp.by_ref.span(), "parameter must not be mutable"),
					));
				}
				if tmp.subpat.is_some() {
					return Err(token_stream_with_error(
						item.to_token_stream(),
						Error::new(tmp.by_ref.span(), "parameter must not have subpattern"),
					));
				}
				if tmp.ident.to_string() == "tnn_operation_context" {
					return Err(token_stream_with_error(
						item.to_token_stream(),
						Error::new(tmp.by_ref.span(), "parameter must not be named 'tnn_operation_context'"),
					));
				}
				tmp.ident.clone()
			}),
		};
		Some((input_name, input_type))
	};

	if let Some((_, ref typ)) = &input_info {
		let typ = typ.clone();
		item.sig.inputs[0] = syn::parse2::<FnArg>(quote!(tnn_operation_context: ::tnn::OpContext<'_, #typ>))
			.expect("could not rewrite input");
	} else {
		item.sig.inputs.push(
			syn::parse2::<FnArg>(quote!(tnn_operation_context: ::tnn::OpContext<'_, ()>))
				.expect("could not rewrite input"),
		);
	}

	// todo(James Bradlee): If function isn't async, don't put the function
	item.sig.asyncness = None;

	let fn_output = match item.sig.output.clone() {
		ReturnType::Default => Type::Tuple(TypeTuple {
			paren_token: Paren(item.sig.output.span()),
			elems: Punctuated::default(),
		}),
		ReturnType::Type(_, ty) => *ty,
	};

	item.sig.output = match item.sig.output {
		ReturnType::Default => default_output_for_function(&item)?,
		ReturnType::Type(arrow, ref it) => ReturnType::Type(arrow, transform_output_for_function(&item, &it)?),
	};

	item.block = Box::new(Block {
		brace_token: Brace(item.block.span()),
		stmts: {
			let mut tmp = Vec::new();
			if let Some((input_name, _)) = &input_info {
				tmp.push(syn::parse2::<Stmt>(quote!(let #input_name = tnn_operation_context.input;)).unwrap());
			}
			tmp.push(Stmt::Expr(
				Expr::Call(ExprCall {
					attrs: vec![],
					func: Box::new(Expr::Path(ExprPath {
						attrs: vec![],
						qself: None,
						path: Path {
							leading_colon: Some(PathSep(item.block.span())),
							segments: {
								let mut seg = Punctuated::new();
								seg.push_value(PathSegment {
									ident: Ident::new("std", item.block.span()),
									arguments: PathArguments::None,
								});
								seg.push_punct(PathSep(item.block.span()));
								seg.push_value(PathSegment {
									ident: Ident::new("boxed", item.block.span()),
									arguments: PathArguments::None,
								});
								seg.push_punct(PathSep(item.block.span()));
								seg.push_value(PathSegment {
									ident: Ident::new("Box", item.block.span()),
									arguments: PathArguments::None,
								});
								seg.push_punct(PathSep(item.block.span()));
								seg.push_value(PathSegment {
									ident: Ident::new("pin", item.block.span()),
									arguments: PathArguments::None,
								});

								seg
							},
						},
					})),
					paren_token: Paren(item.block.span()),
					args: {
						let mut args = Punctuated::new();
						args.push_value(Expr::Async(ExprAsync {
							attrs: vec![],
							async_token: Token![async](item.block.span()),
							capture: Some(Token![move](item.block.span())),
							block: *item.block,
						}));
						args
					},
				}),
				None,
			));
			tmp
		},
	});

	// let fn_vis = item.vis.clone();

	item.vis = match item.vis {
		Visibility::Inherited => Visibility::Inherited,
		Visibility::Restricted(restriction) => Visibility::Restricted(restriction),
		Visibility::Public(token) => match syn::parse2::<Visibility>(quote_spanned!(token.span() => pub(crate))) {
			Err(error) => return Err(token_stream_with_error(item.to_token_stream(), error)),
			Ok(result) => result,
		},
	};

	let fn_name = format_ident!("{}", item.sig.ident.to_string().to_uppercase());
	let fn_name_value = syn::parse2::<Expr>(format!("\"{}\"", fn_name.to_string()).parse().unwrap()).unwrap();
	let fn_input = match &input_info {
		Some((_, typ)) => typ.clone(),
		None => Type::Tuple(TypeTuple {
			paren_token: Paren(Span::call_site()),
			elems: Punctuated::new(),
		}),
	};

	let tokens = quote!(
		pub const #fn_name: ::tnn::Operation<'_, #fn_input, #fn_output> = ::tnn::Operation {
			_phantom: ::std::marker::PhantomData,
			name: #fn_name_value,
			owner: crate::EXTENSION_NAME,
			id: ::tnn::__private::concatcp!(crate::EXTENSION_NAME, "/", #fn_name_value)
		};
	);

	let mut output = item.to_token_stream();

	output.extend(tokens);

	Ok(output)
}
