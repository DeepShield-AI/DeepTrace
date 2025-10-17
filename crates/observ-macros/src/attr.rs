use syn::{
	Attribute, Error, Expr, ExprLit, ExprPath, Lit, Meta, Result, Token, punctuated::Punctuated,
};

pub mod structs {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum FormatKind {
		Kv,
		Space,
		Table,
	}

	pub struct Attrs {
		pub format: Option<FormatKind>,
	}
}

pub mod field {
	use syn::Path;

	#[derive(Default)]
	pub struct Attrs {
		pub key: Option<String>,
		pub index: Option<usize>,
		pub unit: Option<Path>,
		pub parser: Option<Path>,
		pub optional: bool,
		pub docs: String,
	}
}

pub fn get_struct_attrs(input: &[Attribute]) -> Result<structs::Attrs> {
	let mut attrs = structs::Attrs { format: None };

	for attr in input.iter() {
		if attr.path().is_ident("fmt") {
			if attrs.format.is_some() {
				return Err(Error::new_spanned(attr, "duplicate 'fmt' attribute"));
			}
			if let Meta::NameValue(nv) = &attr.meta {
				if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
					attrs.format = Some(match s.value().as_str() {
						"kv" => structs::FormatKind::Kv,
						"space" => structs::FormatKind::Space,
						"table" => structs::FormatKind::Table,
						_ => return Err(Error::new_spanned(s, "invalid fmt")),
					});
				} else {
					return Err(Error::new_spanned(&nv.value, "expected string literal"));
				}
			} else {
				return Err(Error::new_spanned(attr, "expected name-value pair"));
			}
		} else if attr.path().is_ident("doc") {
			// Skip doc comments - they are handled automatically by Rust
		} else {
			return Err(Error::new_spanned(attr, "unknown attribute"));
		}
	}

	Ok(attrs)
}

pub fn get_field_attrs(input: &[Attribute]) -> Result<field::Attrs> {
	let mut attrs = field::Attrs::default();

	for attr in input.iter() {
		if attr.path().is_ident("arg") {
			parse_arg_attribute(attr, &mut attrs)?;
		} else if attr.path().is_ident("doc") &&
			let Meta::NameValue(nv) = &attr.meta &&
			let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value
		{
			if !attrs.docs.is_empty() {
				attrs.docs.push('\n');
			}
			attrs.docs.push_str(&s.value());
		}
	}

	Ok(attrs)
}

fn parse_arg_attribute(attr: &Attribute, attrs: &mut field::Attrs) -> Result<()> {
	let Meta::List(meta_list) = &attr.meta else {
		return Err(Error::new_spanned(attr, "arg attribute must be a list: #[arg(...)]"));
	};

	let nested = meta_list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

	for meta in nested {
		match meta {
			Meta::NameValue(nv) if nv.path.is_ident("unit") => {
				let Expr::Path(ExprPath { path, .. }) = &nv.value else {
					return Err(Error::new_spanned(&nv.value, "unit must be a path"));
				};
				attrs.unit = Some(path.clone());
			},
			Meta::NameValue(nv) if nv.path.is_ident("key") => {
				let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value else {
					return Err(Error::new_spanned(&nv.value, "key must be a string literal"));
				};
				attrs.key = Some(s.value());
			},
			Meta::NameValue(nv) if nv.path.is_ident("index") => {
				let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = &nv.value else {
					return Err(Error::new_spanned(&nv.value, "index must be an integer"));
				};
				attrs.index = Some(i.base10_parse()?);
			},
			Meta::NameValue(nv) if nv.path.is_ident("with") => {
				let Expr::Path(ExprPath { path, .. }) = &nv.value else {
					return Err(Error::new_spanned(&nv.value, "with must be a path"));
				};
				attrs.parser = Some(path.clone());
			},
			Meta::Path(path) if path.is_ident("optional") => {
				attrs.optional = true;
			},
			_ => return Err(Error::new_spanned(&meta, "unknown parameter in arg attribute")),
		}
	}

	Ok(())
}
