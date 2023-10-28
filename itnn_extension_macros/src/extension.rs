//
// DISCLAIMER
//
// I know this code is awful, welcome to the real world.
//
// Parsing Rust syntax trees is an awful long process,
// and takes ungodly many lines of codes to do.
//
// It works, it might not be the best written code,
// but it works. It migt take a lot of time to actually
// understand what's happening here, so you may as well
// go do something else.
//

use std::{borrow::BorrowMut, collections::HashSet, fmt::Display};

use proc_macro2::{Delimiter, Ident, Punct, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::{Error, LitStr};

fn synerr<T>(span: Span, error: impl Display) -> Result<T, TokenStream> {
	let mut tokens = TokenStream::new();
	tokens.extend(Error::new(span, error).into_compile_error());
	Err(tokens)
}

fn with_synerr<T>(err: Error) -> Result<T, TokenStream> {
	let mut tokens = TokenStream::new();
	tokens.extend(err.into_compile_error());
	Err(tokens)
}

pub(crate) struct ExtensionDef {
	name: Option<LitStr>,
	version: Option<LitStr>,
	dependencies: Option<Vec<DependencyDef>>,
	ops: Option<Vec<OperationDef>>,
}

enum ParseState {
	Key,
	Eq(ParseProp),
	Parse(ParseProp),
	Sep,
}

enum ParseProp {
	Name,
	Version,
	Dependencies(DependenciesState),
	Operations(OperationsState),
}

struct DependenciesState {
	definitions: Option<Vec<DependencyDef>>,
	seen: HashSet<String>,

	property: Option<Ident>,
	eq: Option<Punct>,
	optional: Option<Ident>,
	version: Option<LitStr>,
}

impl DependenciesState {
	fn new() -> Self {
		Self {
			definitions: Some(Vec::new()),
			seen: HashSet::new(),
			property: None,
			eq: None,
			optional: None,
			version: None,
		}
	}

	fn expects_property(&self) -> bool {
		self.property.is_none() && self.version.is_none()
	}

	fn expects_equal(&self) -> bool {
		self.property.is_some() && self.eq.is_none()
	}

	fn expects_optional(&self) -> bool {
		self.eq.is_some() && self.version.is_none()
	}

	fn expects_version(&self) -> bool {
		self.eq.is_some()
	}

	fn expects_separator(&self) -> bool {
		self.version.is_some()
	}

	/// returns false if not complete parsing value, or true when value is parsed.
	fn handle_token(&mut self, token: &TokenTree) -> Result<(), TokenStream> {
		let group = match &token {
			TokenTree::Group(grp) => grp,
			_ => return synerr(token.span(), "unexpected token, expected group"),
		};
		if group.delimiter() != Delimiter::Brace {
			return synerr(group.span(), "unexpected punctuation, expected punctuation '{'");
		}
		for token in group.stream() {
			if self.expects_property() {
				match token {
					TokenTree::Ident(ident) => {
						let name = ident.to_string();
						if self.seen.contains(&name) {
							return synerr(ident.span(), "dependency already declared");
						}
						self.seen.insert(name);
						self.property = Some(ident)
					}
					_ => return synerr(token.span(), "expected dependency name"),
				}
				continue;
			}
			if self.expects_equal() {
				match token {
					TokenTree::Punct(punct) => {
						if punct.to_string() != "=" {
							return synerr(punct.span(), "expected punctuation '='");
						}
						self.eq = Some(punct);
					}
					_ => return synerr(token.span(), "expected punctuation '='"),
				}
				continue;
			}
			if self.expects_version() {
				match token {
					TokenTree::Ident(ident) => {
						if !self.expects_optional() {
							return synerr(ident.span(), "unexpected identifier, expected dependency version");
						}
						if ident.to_string() != "optional" {
							return synerr(ident.span(), "unexpected identifier, expected identifier 'optional'");
						}
						self.optional = Some(ident);
					}
					TokenTree::Literal(lit) => {
						let litstr = match syn::parse2::<LitStr>(lit.to_token_stream()) {
							Err(err) => {
								return synerr(
									lit.span(),
									format!("expected a literal string with dependency version: {}", err),
								)
							}
							Ok(litstr) => litstr,
						};
						let matcher = litstr.value();
						if let Err(err) = semver::VersionReq::parse(&matcher) {
							return synerr(lit.span(), format!("Invalid version matcher: {}", err));
						}
						if let Some(deps) = self.definitions.borrow_mut() {
							deps.push(DependencyDef {
								name_ident: self.property.take().unwrap(),
								optional_ident: self.optional.take(),
								version_req_literal: litstr.clone(),
							});
						}
						self.version = Some(litstr);
						self.property = None;
						self.eq = None;
						self.optional = None;
					}
					_ => {
						if self.expects_optional() {
							return synerr(
								token.span(),
								"unexpected token, expected identifier 'optional' or dependency version",
							);
						} else {
							return synerr(token.span(), "unexpected token, expected dependency version");
						}
					}
				}
				continue;
			}
			if self.expects_separator() {
				match token {
					TokenTree::Punct(punct) => {
						if punct.to_string() != "," {
							return synerr(
								punct.span(),
								format!(
									"unexpected punctuation '{}', expected punctuation ','",
									punct.to_string()
								),
							);
						}
						self.version = None;
					}
					_ => return synerr(token.span(), "unexpected token, expected punctuation ','"),
				}
				continue;
			}
		}
		Ok(())
	}
}

struct DependencyDef {
	name_ident: Ident,
	optional_ident: Option<Ident>,
	version_req_literal: LitStr,
}

struct OperationsState {
	definitions: Option<Vec<OperationDef>>,
	fn_ref: TokenStream,
	def_ref: TokenStream,
	ident: Option<Ident>,
	colon1: Option<Punct>,
	colon2: Option<Punct>,
}

impl OperationsState {
	pub fn new() -> Self {
		Self {
			definitions: Some(Vec::new()),

			fn_ref: TokenStream::new(),
			def_ref: TokenStream::new(),
			ident: None,
			colon1: None,
			colon2: None,
		}
	}

	fn expects_ident(&self) -> bool {
		(self.ident.is_none() && !self.colon1.is_some()) || self.colon2.is_some()
	}

	fn expects_colon1(&self) -> bool {
		self.ident.is_some() && self.colon1.is_none()
	}

	fn expects_colon2(&self) -> bool {
		self.colon1.is_some()
	}

	fn commit(&mut self) {
		let ident1 = self.ident.take().unwrap();
		let ident2 = format_ident!("{}", ident1.clone().to_string().to_uppercase());

		self.fn_ref.extend(quote! { #ident1 });
		self.def_ref.extend(quote! { #ident2 });

		let fn_ref = self.fn_ref.clone();
		let def_ref = self.def_ref.clone();

		self.fn_ref = TokenStream::new();
		self.def_ref = TokenStream::new();

		if let Some(defs) = self.definitions.borrow_mut() {
			defs.push(OperationDef {
				function_handle: fn_ref,
				definition_handle: def_ref,
			})
		}
	}

	pub fn handle_token(&mut self, token: &TokenTree) -> Result<(), TokenStream> {
		let group = match &token {
			TokenTree::Group(grp) => grp,
			_ => return synerr(token.span(), "unexpected token, expected group"),
		};
		if group.delimiter() != Delimiter::Bracket {
			return synerr(group.span(), "unexpected punctuation, expected punctuation '['");
		}
		for token in group.stream() {
			if self.expects_ident() {
				if self.colon2.is_some() {
					let colon1 = self.colon1.take().unwrap();
					let colon2 = self.colon2.take().unwrap();

					let stream = quote! { #colon1 #colon2 };

					self.fn_ref.extend(stream.clone());
					self.def_ref.extend(stream);
				}

				match token {
					TokenTree::Ident(ident) => self.ident = Some(ident),
					_ => return synerr(token.span(), "unexpected token, expected identifier"),
				};

				continue;
			}

			if self.expects_colon1() {
				match token {
					TokenTree::Punct(punct) => match punct.as_char() {
						',' => self.commit(),
						':' => {
							self.colon1 = Some(punct);

							let ident = self.ident.take().unwrap();
							let stream = quote! { #ident };

							self.fn_ref.extend(stream.clone());
							self.def_ref.extend(stream);
						}
						chr => {
							return synerr(
								punct.span(),
								format!("unexpected punctuation '{}', expected '::' or ','", chr),
							)
						}
					},
					_ => return synerr(token.span(), "unexpected token, expected punctuation '::' or ','"),
				};

				continue;
			}

			if self.expects_colon2() {
				match token {
					TokenTree::Punct(punct) => match punct.as_char() {
						':' => self.colon2 = Some(punct),
						chr => return synerr(punct.span(), format!("unexpected punctuation '{}', expected '::'", chr)),
					},
					_ => return synerr(token.span(), "unexpected token, expected punctuation '::'"),
				};

				continue;
			}
		}

		if self.colon1.is_some() || self.colon2.is_some() {
			return synerr(
				self.colon2.clone().or(self.colon1.clone()).unwrap().span(),
				"unexpected end of input",
			);
		}

		if self.ident.is_some() {
			self.commit();
		}

		println!("");
		println!("");
		println!("");
		println!("Operation definitions:");
		println!("");
		println!("");
		println!("");
		println!("{:?}", self.definitions);
		println!("");
		println!("");
		println!("");
		Ok(())
	}
}

#[derive(Debug)]
struct OperationDef {
	function_handle: TokenStream,
	definition_handle: TokenStream,
}

impl ParseProp {
	pub fn from_ident(ident: &Ident) -> Result<ParseProp, TokenStream> {
		Ok(match ident.to_string().as_str() {
			"name" => ParseProp::Name,
			"version" => ParseProp::Version,
			"dependencies" => ParseProp::Dependencies(DependenciesState::new()),
			"ops" => ParseProp::Operations(OperationsState::new()),
			_ => {
				return synerr(
					ident.span(),
					format!("Property '{}' is not valid in this context", ident.to_string()),
				);
			}
		})
	}
}

pub(crate) fn parse(tokens: TokenStream) -> Result<ExtensionDef, TokenStream> {
	let mut def = ExtensionDef {
		name: None,
		version: None,
		dependencies: None,
		ops: None,
	};

	let mut last_token: Option<TokenTree> = None;
	let mut global_state = ParseState::Key;

	for token in tokens.clone() {
		match global_state {
			ParseState::Key => match token {
				TokenTree::Ident(ref identifier) => {
					let prop = ParseProp::from_ident(identifier)?;
					match &prop {
						ParseProp::Name => {
							if def.name.is_some() {
								return synerr(token.span(), "Field is already defined");
							}
						}
						ParseProp::Version => {
							if def.version.is_some() {
								return synerr(token.span(), "Field is already defined");
							}
						}
						ParseProp::Dependencies(_) => {
							if def.dependencies.is_some() {
								return synerr(token.span(), "Field is already defined");
							}
						}
						ParseProp::Operations(_) => {
							if def.ops.is_some() {
								return synerr(token.span(), "Field is already defined");
							}
						}
					}
					global_state = ParseState::Eq(prop);
				}
				_ => return synerr(token.span(), "Unexpected token, expected identifier"),
			},
			ParseState::Eq(parse_state) => match token {
				TokenTree::Punct(ref token) => {
					if token.to_string() != "=" {
						return synerr(token.span(), "Unexpected token, expected token '='");
					}
					global_state = ParseState::Parse(parse_state);
				}
				_ => return synerr(token.span(), "Unexpected token, expected token '='"),
			},
			ParseState::Sep => match &token {
				TokenTree::Punct(ref punct) => {
					if punct.to_string() != "," {
						return synerr(
							token.span(),
							format!("Unexepcted punctuation '{}', expected punctuation ','", punct.as_char()),
						);
					}
					global_state = ParseState::Key;
				}
				_ => return synerr(token.span(), "Unexepcted token, expected punctuation ','"),
			},
			ParseState::Parse(prop) => match prop {
				ParseProp::Name => match &token {
					TokenTree::Literal(lit) => match syn::parse2::<LitStr>(lit.to_token_stream()) {
						Ok(litstr) => {
							def.name = Some(litstr);
							global_state = ParseState::Sep;
						}
						Err(err) => {
							return with_synerr(err);
						}
					},
					_ => return synerr(token.span(), "Unexpected token, expected a string literal"),
				},
				ParseProp::Version => match &token {
					TokenTree::Literal(lit) => match syn::parse2::<LitStr>(lit.to_token_stream()) {
						Ok(litstr) => {
							let lit = litstr.to_token_stream().to_string();
							match semver::Version::parse(&lit[1..lit.len() - 1]) {
								Err(err) => return synerr(token.span(), format!("invalid version: {}", err)),
								_ => {}
							}
							def.version = Some(litstr);
							global_state = ParseState::Sep;
						}
						Err(err) => {
							return with_synerr(err);
						}
					},
					_ => return synerr(token.span(), "Unexpected token, expected a string literal"),
				},
				ParseProp::Dependencies(mut dependencies_state) => {
					dependencies_state.handle_token(&token)?;
					def.dependencies = Some(dependencies_state.definitions.unwrap());
					global_state = ParseState::Sep;
				}
				ParseProp::Operations(mut operations_state) => {
					operations_state.handle_token(&token)?;
					def.ops = Some(operations_state.definitions.unwrap());
					global_state = ParseState::Sep;
				}
			},
		};
		last_token = Some(token);
	}

	match &global_state {
		ParseState::Key => {}
		ParseState::Eq(_) => {
			return synerr(last_token.unwrap().span(), "Unexpected end of input, missing token '='");
		}
		ParseState::Parse(_) => {
			return synerr(last_token.unwrap().span(), "Unexpected end of input, missing value");
		}
		ParseState::Sep => {}
	}

	return Ok(def);
}

pub(crate) fn expand(ext: ExtensionDef) -> TokenStream {
	let name = match ext.name {
		None => quote!(env!("CARGO_PKG_NAME")),
		Some(name) => name.into_token_stream(),
	};

	let version = match ext.version {
		None => quote!(env!("CARGO_PKG_VERSION")),
		Some(version) => version.into_token_stream(),
	};

	let mut dependency_stream = TokenStream::new();
	for dep in ext.dependencies.unwrap() {
		dependency_stream.extend(expand_dependency(dep));
	}

	let init = expand_init(ext.ops.or(Some(Vec::new())).unwrap());

	let result = quote!(
		pub const EXTENSION_NAME: &'static str = #name;

		#[no_mangle]
		pub(crate) static EXTENSION: ::tnn::Extension = ::tnn::Extension {
			name: EXTENSION_NAME,
			version: #version,
			dependencies: &[#dependency_stream],
			init: #init
		};
	);

	result
}

fn expand_optional_dependency(dep: DependencyDef) -> TokenStream {
	let name = dep.name_ident;
	let version = dep.version_req_literal;
	quote!(
		::tnn::Dependency::Optional {
			name: stringify!(#name),
			version_matcher: #version
		},
	)
}

fn expand_required_dependency(dep: DependencyDef) -> TokenStream {
	let name = dep.name_ident;
	let version = dep.version_req_literal;
	quote!(
		::tnn::Dependency::Required {
			name: stringify!(#name),
			version_matcher: #version
		},
	)
}

fn expand_dependency(dep: DependencyDef) -> TokenStream {
	if dep.optional_ident.is_some() {
		return expand_optional_dependency(dep);
	} else {
		return expand_required_dependency(dep);
	}
}

fn expand_init(ops: Vec<OperationDef>) -> TokenStream {
	if ops.len() == 0 {
		return quote! { None };
	}

	let mut add_ops: Vec<TokenStream> = Vec::new();

	for op in ops {
		let def_handle = op.definition_handle;
		let fun_handle = op.function_handle;
		add_ops.push(quote! {
			ctx.add_operation(&#def_handle, &#fun_handle).await?;
		});
	}

	quote! {
		Some(&|ctx| Box::pin(async move {
			#( #add_ops )*
			Ok(())
		}))
	}
}
