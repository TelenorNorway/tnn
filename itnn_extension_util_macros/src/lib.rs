use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use quote::TokenStreamExt;
use syn::{spanned::Spanned, Error};

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
	tokens.extend(error.into_compile_error());
	tokens
}

enum LastItem {
	Ident(Ident),
	Punctuation(Punct),
	Sep(Punct),
	None,
}

#[proc_macro]
pub fn extension_operation_function_reference(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input: TokenStream = input.into();
	let mut output = TokenStream::new();

	let mut last = LastItem::None;

	for value in input.clone() {
		let last_item = last;
		match &last_item {
			LastItem::None => match value {
				TokenTree::Group(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected group, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Literal(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected literal, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Ident(ident) => {
					last = LastItem::Ident(ident);
				}
				TokenTree::Punct(ref punct) => {
					if punct.as_char() != ':' {
						return token_stream_with_error(
							input,
							Error::new(
								value.span(),
								format!("unexpected punctuation '{}', expected punctuation ':'", punct.as_char()),
							),
						)
						.into();
					}
					last = LastItem::Punctuation(punct.clone());
				}
			},
			LastItem::Punctuation(_) => match value {
				TokenTree::Group(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected group, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Literal(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected literal, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Ident(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected identifier, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Punct(ref punct) => {
					if punct.as_char() != ':' {
						return token_stream_with_error(
							input,
							Error::new(
								value.span(),
								format!("unexpected punctuation '{}', expected punctuation ':'", punct.as_char()),
							),
						)
						.into();
					}
					last = LastItem::Sep(punct.clone());
				}
			},
			LastItem::Sep(_) => match value {
				TokenTree::Group(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected group, expected identifier"),
					)
					.into()
				}
				TokenTree::Literal(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected literal, expected identifier"),
					)
					.into()
				}
				TokenTree::Ident(ident) => last = LastItem::Ident(ident),
				TokenTree::Punct(ref punct) => {
					return token_stream_with_error(
						input,
						Error::new(
							value.span(),
							format!("unexpected punctuation '{}', expected identifier", punct.as_char()),
						),
					)
					.into();
				}
			},
			LastItem::Ident(_) => match value {
				TokenTree::Group(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected group, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Literal(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected literal, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Ident(_) => {
					return token_stream_with_error(
						input,
						Error::new(value.span(), "unexpected identifier, expected punctuation ':'"),
					)
					.into()
				}
				TokenTree::Punct(ref punct) => {
					if punct.as_char() != ':' {
						return token_stream_with_error(
							input,
							Error::new(
								value.span(),
								format!("unexpected punctuation '{}', expected punctuation ':'", punct.as_char()),
							),
						)
						.into();
					}
					last = LastItem::Punctuation(punct.clone());
				}
			},
		};
		match &last_item {
			LastItem::Ident(ident) => output.append(ident.clone()),
			LastItem::Punctuation(punct) => output.append(punct.clone()),
			LastItem::Sep(punct) => output.append(punct.clone()),
			LastItem::None => {}
		}
	}

	match &last {
		LastItem::Ident(ident) => {
			output.append(Ident::new(&ident.to_string().to_uppercase(), ident.span()));
		}
		LastItem::Punctuation(value) => {
			return token_stream_with_error(
				input,
				Error::new(value.span(), "unexpected end of input, expected punctuation ':'"),
			)
			.into()
		}
		LastItem::Sep(value) => {
			return token_stream_with_error(
				input,
				Error::new(value.span(), "unexpected end of input, expected identifier"),
			)
			.into()
		}
		LastItem::None => {
			return token_stream_with_error(
				input.clone(),
				Error::new(
					input.span(),
					"unexpected end of input, expected identifier or punctuation ':'",
				),
			)
			.into()
		}
	}

	output.into()
}
