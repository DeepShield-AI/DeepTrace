//! # ProcParser - A derive macro for parsing /proc files
//!
//! This crate provides a derive macro for automatically generating parsers for
//! Linux /proc files like /proc/meminfo, /proc/stat, /proc/diskstats,
//! /proc/net/dev, etc.
//!
//! ## Features
//! - Automatic field parsing with nom-based parsers
//! - Support for optional fields (different kernel versions)
//! - Unit conversion support (uom crate integration)
//! - Custom parsing functions
//! - Multiple format support (key-value, space-separated, table)
//!
//! ## Attributes
//! - `#[fmt = "kv" | "space" | "table"]` - parsing format
//! - `#[key = "..."]` - custom field key
//! - `#[index = N]` - column index for table format
//! - `#[unit = "..."]` - unit specification
//! - `#[with = "path::to::parser"]` - custom parser function
//! - `#[optional]` - mark field as optional
//!
//! ## Visibility Control
//! Generated methods inherit the visibility of the struct definition:
//! - `struct MyStruct` → `fn get_field()` (private)
//! - `pub struct MyStruct` → `pub fn get_field()` (public)
//! - `pub(crate) struct MyStruct` → `pub(crate) fn get_field()` (crate-visible)

use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

mod ast;
mod attr;
mod proc_parser;
mod types;
mod utils;

#[proc_macro_derive(ProcParser, attributes(fmt, arg))]
pub fn proc_parser_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	proc_parser::derive(&input).unwrap_or_else(Error::into_compile_error).into()
}
