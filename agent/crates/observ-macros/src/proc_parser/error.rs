//! Error type generation for ProcParser.
//!
//! Generates a custom error type following thiserror patterns.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

/// Generate error type and Display/Error implementations
pub fn generate_error_type(struct_name: &Ident) -> TokenStream {
	let error_name = format_ident!("{}ParseError", struct_name);

	quote! {
		/// Error type for parsing failures.
		#[derive(Debug, Clone)]
		pub enum #error_name {
			/// Failed to parse a field value
			ParseField {
				/// Name of the field that failed to parse
				field: &'static str,
				/// The value that failed to parse
				value: ::std::string::String,
				/// Description of expected format
				expected: &'static str,
			},
			/// Invalid line format
			InvalidFormat {
				/// Line number (1-indexed)
				line: usize,
				/// The problematic line content
				content: ::std::string::String,
			},
			/// Generic error message
			Message(::std::string::String),
		}

		impl ::std::fmt::Display for #error_name {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				match self {
					Self::ParseField { field, value, expected } => {
						write!(f, "failed to parse field '{}': got '{}', expected {}", field, value, expected)
					}
					Self::InvalidFormat { line, content } => {
						write!(f, "invalid format at line {}: '{}'", line, content)
					}
					Self::Message(msg) => write!(f, "{}", msg),
				}
			}
		}

		impl ::std::error::Error for #error_name {}

		impl From<&str> for #error_name {
			fn from(msg: &str) -> Self {
				Self::Message(msg.to_string())
			}
		}

		impl From<::std::string::String> for #error_name {
			fn from(msg: ::std::string::String) -> Self {
				Self::Message(msg)
			}
		}
	}
}

/// Generate error type name for a struct
pub fn error_type_name(struct_name: &Ident) -> Ident {
	format_ident!("{}ParseError", struct_name)
}
