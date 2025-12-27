//! Attribute parsing for ProcParser derive macro using darling.
//!
//! This module uses darling for declarative attribute parsing, providing:
//! - Automatic error messages with source spans
//! - "Did you mean" suggestions for typos
//! - Type-safe attribute extraction

use darling::{FromDeriveInput, FromField, FromMeta, ast::Data};
use syn::Path;

/// Parsing format for /proc files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatKind {
	/// Key-value pairs (e.g., /proc/meminfo: "MemTotal: 1234 kB")
	#[default]
	Kv,
	/// Space-separated key-value (e.g., /proc/vmstat)
	Space,
	/// Table format (e.g., /proc/diskstats)
	Table,
}

impl FromMeta for FormatKind {
	fn from_string(value: &str) -> darling::Result<Self> {
		match value {
			"kv" => Ok(FormatKind::Kv),
			"space" => Ok(FormatKind::Space),
			"table" => Ok(FormatKind::Table),
			"key-value" | "keyvalue" => Err(darling::Error::custom(
				"unknown format `key-value`, did you mean `kv`?",
			)),
			"tab" | "tabular" => Err(darling::Error::custom(
				"unknown format `tabular`, did you mean `table`?",
			)),
			"space-separated" | "whitespace" => Err(darling::Error::custom(
				"unknown format, did you mean `space`?",
			)),
			_ => Err(darling::Error::custom(format!(
				"unknown format `{value}`, expected one of: `kv`, `space`, `table`"
			))),
		}
	}
}

/// Struct-level input parsed by darling
#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named))]
pub struct StructInput {
	/// The struct identifier
	pub ident: syn::Ident,
	/// The struct visibility
	pub vis: syn::Visibility,
	/// Parsed fields
	pub data: Data<(), FieldInput>,
}

/// Field-level attributes parsed by darling
#[derive(Debug, FromField)]
#[darling(attributes(arg))]
pub struct FieldInput {
	/// Field identifier (None for tuple structs)
	pub ident: Option<syn::Ident>,
	/// Field visibility
	pub vis: syn::Visibility,
	/// Field type
	pub ty: syn::Type,

	/// Custom key name for kv/space format
	#[darling(default)]
	pub key: Option<String>,
	/// Column index for table format
	#[darling(default)]
	pub index: Option<usize>,
	/// UOM unit path
	#[darling(default)]
	pub unit: Option<Path>,
	/// Custom parser function path (renamed from "with" in attributes)
	#[darling(default, rename = "with")]
	pub parser: Option<Path>,
	/// Whether field is optional
	#[darling(default)]
	pub optional: bool,
}
