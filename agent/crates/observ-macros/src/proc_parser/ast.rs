//! AST structures for ProcParser derive macro.
//!
//! This module provides processed AST structures ready for code generation,
//! built on top of darling's parsed input.

use super::{attr::{FieldInput, FormatKind, StructInput}, types};
use crate::common;
use darling::FromDeriveInput;
use darling::FromMeta;
use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, Path, Type, Visibility};

/// Processed struct information ready for code generation
pub struct Struct {
	/// The struct identifier
	pub ident: Ident,
	/// The struct visibility
	pub visibility: Visibility,
	/// Parsing format (kv, space, table)
	pub format: FormatKind,
	/// Processed fields
	pub fields: Vec<Field>,
}

/// Processed field information ready for code generation
pub struct Field {
	/// Field identifier
	pub ident: Ident,
	/// Field visibility
	pub visibility: Visibility,
	/// Field type
	pub ty: Type,
	/// Custom key name for kv/space format
	pub key: Option<String>,
	/// Column index for table format
	pub index: Option<usize>,
	/// UOM unit path
	pub unit: Option<Path>,
	/// Custom parser function path
	pub parser: Option<Path>,
	/// Whether field is marked optional
	pub optional: bool,
	/// Whether the type is Option<T>
	pub is_option_type: bool,
	/// Inner type if Option<T>
	pub inner_type: Option<Type>,
	/// Generated UOM conversion code
	pub uom_conversion_code: Option<TokenStream>,
}

impl Struct {
	/// Parse from syn::DeriveInput using darling
	pub fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
		let parsed = StructInput::from_derive_input(input)?;

		// Extract format from #[fmt = "..."] attribute
		let format = Self::extract_format(input)?;

		// Process fields
		let fields = parsed
			.data
			.take_struct()
			.ok_or_else(|| darling::Error::unsupported_shape("only named structs are supported"))?
			.fields
			.into_iter()
			.map(Field::from_field_input)
			.collect::<darling::Result<Vec<_>>>()?;

		Ok(Self { ident: parsed.ident, visibility: parsed.vis, format, fields })
	}

	/// Extract format from #[fmt = "..."] attribute
	fn extract_format(input: &DeriveInput) -> darling::Result<FormatKind> {
		for attr in &input.attrs {
			if attr.path().is_ident("fmt") {
				if let syn::Meta::NameValue(nv) = &attr.meta {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
						return FormatKind::from_string(&s.value())
							.map_err(|e| e.with_span(s));
					}
					return Err(darling::Error::custom(
						"expected string literal, e.g. #[fmt = \"kv\"]"
					).with_span(&nv.value));
				}
				return Err(darling::Error::custom(
					"expected #[fmt = \"...\"], not #[fmt(...)] or #[fmt]"
				).with_span(&attr.meta));
			}
		}
		Ok(FormatKind::default())
	}
}

impl Field {
	/// Convert from darling's FieldInput to processed Field
	fn from_field_input(input: FieldInput) -> darling::Result<Self> {
		let ident = input
			.ident
			.ok_or_else(|| darling::Error::custom("only named fields are supported"))?;

		let inner_type = common::extract_option_inner_type(&input.ty);
		let is_option_type = inner_type.is_some();

		// Generate UOM conversion code
		let uom_conversion_code = {
			let ty = inner_type.as_ref().unwrap_or(&input.ty);
			types::generate_uom_conversion(ty, input.unit.as_ref())
		};

		Ok(Self {
			ident,
			visibility: input.vis,
			ty: input.ty,
			key: input.key,
			index: input.index,
			unit: input.unit,
			parser: input.parser,
			optional: input.optional,
			is_option_type,
			inner_type,
			uom_conversion_code,
		})
	}
}
