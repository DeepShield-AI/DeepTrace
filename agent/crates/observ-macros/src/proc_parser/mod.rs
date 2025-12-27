//! ProcParser derive macro for parsing Linux /proc files.
//!
//! This module provides a derive macro for automatically generating parsers for
//! Linux /proc files like /proc/meminfo, /proc/stat, /proc/diskstats, etc.
//!
//! ## Supported Formats
//! - `kv` - Key-value pairs (e.g., /proc/meminfo)
//! - `space` - Space-separated values (e.g., /proc/vmstat)
//! - `table` - Table format with SIMD acceleration via memchr (e.g., /proc/diskstats)
//!
//! ## Struct Attributes
//! - `#[fmt = "kv" | "space" | "table"]` - parsing format
//!
//! ## Field Attributes
//! - `#[arg(key = "...")]` - custom field key
//! - `#[arg(index = N)]` - column index for table format
//! - `#[arg(unit = path)]` - unit specification for UOM types
//! - `#[arg(with = path::to::parser)]` - custom parser function
//! - `#[arg(optional)]` - mark field as optional

mod ast;
mod attr;
mod codegen;
mod error;
mod types;
mod utils;
mod valid;

pub use ast::Struct;
use proc_macro2::TokenStream;
use syn::DeriveInput;

/// Entry point for the ProcParser derive macro
pub fn derive(input: &DeriveInput) -> Result<TokenStream, darling::Error> {
	let s = Struct::from_derive_input(input)?;
	s.validate()?;
	Ok(codegen::generate(&s))
}
