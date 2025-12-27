//! Code generation for ProcParser derive macro.

use super::{ast::{Field, Struct}, attr::FormatKind, error};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

/// Threshold for using PHF instead of match. PHF has overhead for small maps.
const PHF_THRESHOLD: usize = 8;

/// Check if PHF feature is enabled
#[inline]
fn use_phf() -> bool {
	cfg!(feature = "phf")
}

/// Check if SIMD feature is enabled
#[inline]
fn use_simd() -> bool {
	cfg!(feature = "simd")
}

/// Generate line iterator code
fn generate_line_iterator() -> TokenStream {
	// Generate SIMD or fallback line finding based on feature
	let find_newline = if use_simd() {
		quote! {
			::memchr::memchr(b'\n', &self.bytes[self.pos..])
				.map(|i| self.pos + i)
				.unwrap_or(self.bytes.len())
		}
	} else {
		quote! {
			self.bytes[self.pos..].iter()
				.position(|&b| b == b'\n')
				.map(|i| self.pos + i)
				.unwrap_or(self.bytes.len())
		}
	};

	quote! {
		struct LineIter<'a> {
			bytes: &'a [u8],
			pos: usize,
		}

		impl<'a> LineIter<'a> {
			#[inline]
			fn new(input: &'a str) -> Self {
				Self { bytes: input.as_bytes(), pos: 0 }
			}
		}

		impl<'a> Iterator for LineIter<'a> {
			type Item = &'a [u8];

			#[inline]
			fn next(&mut self) -> Option<Self::Item> {
				loop {
					if self.pos >= self.bytes.len() {
						return None;
					}

					let end = #find_newline;

					// Trim leading whitespace
					let mut line_start = self.pos;
					while line_start < end && (self.bytes[line_start] == b' ' || self.bytes[line_start] == b'\t') {
						line_start += 1;
					}
					// Trim trailing whitespace
					let mut line_end = end;
					while line_end > line_start && matches!(self.bytes[line_end - 1], b' ' | b'\t' | b'\r') {
						line_end -= 1;
					}

					self.pos = end + 1;

					// Skip empty lines and comments inline (loop instead of recursion)
					let line = &self.bytes[line_start..line_end];
					if !line.is_empty() && line[0] != b'#' {
						return Some(line);
					}
				}
			}
		}
	}
}

/// Generate the impl block for a struct
pub fn generate(s: &Struct) -> TokenStream {
	let struct_name = &s.ident;
	let format = s.format;

	let error_type = error::generate_error_type(struct_name);
	let error_name = error::error_type_name(struct_name);
	let field_inits = generate_field_inits(s);
	let getters = generate_getters(s);

	// Generate PHF map outside impl block if needed
	let (phf_map, parser) = generate_parse_impl(s, format, &error_name, &field_inits);

	quote! {
		#error_type

		#phf_map

		#[automatically_derived]
		#[allow(irrefutable_let_patterns, dead_code)]
		impl #struct_name {
			#parser
			#(#getters)*
		}
	}
}

fn generate_parse_impl(
	s: &Struct,
	format: FormatKind,
	error_name: &syn::Ident,
	field_inits: &[TokenStream],
) -> (TokenStream, TokenStream) {
	match format {
		FormatKind::Kv => generate_kv_parser(s, field_inits, error_name),
		FormatKind::Space => generate_space_parser(s, field_inits, error_name),
		FormatKind::Table => (quote! {}, generate_table_parser(s, field_inits, error_name)),
	}
}

fn generate_field_inits(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let name = &field.ident;
			let span = field.ident.span();

			if field.is_option_type {
				quote_spanned! { span => #name: None }
			} else {
				quote_spanned! { span => #name: Default::default() }
			}
		})
		.collect()
}

fn generate_getters(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let name = &field.ident;
			let span = field.ident.span();
			let ty = &field.ty;
			let vis = &s.visibility;

			quote_spanned! { span =>
				paste::paste! {
					#vis fn [<get_ #name>](&self) -> &#ty {
						&self.#name
					}
				}
			}
		})
		.collect()
}

/// Configuration for line-based parser generation
struct LineParserConfig {
	/// Line parser function definition
	line_parser_fn: TokenStream,
	/// Pattern to destructure parsed line result
	line_pattern: TokenStream,
}

/// Generate a line-based parser (shared logic for KV and Space formats)
fn generate_line_based_parser(
	s: &Struct,
	field_inits: &[TokenStream],
	error_name: &syn::Ident,
	config: LineParserConfig,
	phf_lookup_fn: impl Fn(&Struct) -> (TokenStream, TokenStream),
	match_arms_fn: impl Fn(&Struct) -> Vec<TokenStream>,
) -> (TokenStream, TokenStream) {
	let field_count = s.fields.len();
	// Use PHF only if feature is enabled AND field count exceeds threshold
	let should_use_phf = use_phf() && field_count >= PHF_THRESHOLD;

	let (phf_map_def, field_handlers) = if should_use_phf {
		phf_lookup_fn(s)
	} else {
		(quote! {}, quote! {})
	};

	let match_arms = if !should_use_phf { match_arms_fn(s) } else { vec![] };

	let lookup_code = if should_use_phf {
		quote! {
			if let Some(&field_id) = FIELD_MAP.get(key) {
				#field_handlers
			}
		}
	} else {
		quote! {
			match key {
				#(#match_arms)*
				_ => {}
			}
		}
	};

	let line_iter = generate_line_iterator();
	let phf_static_inside = if should_use_phf { phf_map_def } else { quote! {} };
	let line_parser_fn = &config.line_parser_fn;
	let line_pattern = &config.line_pattern;

	let parse_impl = quote! {
		/// Parse from /proc file content.
		pub fn parse(input: &str) -> Result<Self, #error_name> {
			#line_iter
			#phf_static_inside
			#line_parser_fn

			let mut result = Self { #(#field_inits,)* };
			let mut matched = 0usize;

			for line in LineIter::new(input) {
				if let Some(#line_pattern) = parse_line(line) {
					#lookup_code
				}
				if matched >= #field_count {
					break;
				}
			}

			Ok(result)
		}
	};

	(quote! {}, parse_impl)
}

/// KV Format Parser with Perfect Hashing for large structs
fn generate_kv_parser(
	s: &Struct,
	field_inits: &[TokenStream],
	error_name: &syn::Ident,
) -> (TokenStream, TokenStream) {
	// Generate SIMD or fallback colon finding
	let find_colon = if use_simd() {
		quote! { ::memchr::memchr(b':', line)? }
	} else {
		quote! { line.iter().position(|&b| b == b':')? }
	};

	let config = LineParserConfig {
		line_parser_fn: quote! {
			/// Fast KV line parser - extracts key, value, and optional unit
			#[inline]
			fn parse_line(line: &[u8]) -> Option<(&str, &str, Option<&str>)> {
				let colon_pos = #find_colon;
				let key = unsafe { ::std::str::from_utf8_unchecked(&line[..colon_pos]) };

				let mut pos = colon_pos + 1;
				while pos < line.len() && (line[pos] == b' ' || line[pos] == b'\t') {
					pos += 1;
				}

				let value_start = pos;
				while pos < line.len() && (line[pos].is_ascii_digit() || line[pos] == b'.') {
					pos += 1;
				}
				if pos == value_start {
					return None;
				}
				let value = unsafe { ::std::str::from_utf8_unchecked(&line[value_start..pos]) };

				while pos < line.len() && (line[pos] == b' ' || line[pos] == b'\t') {
					pos += 1;
				}

				let unit = if pos < line.len() && line[pos].is_ascii_alphabetic() {
					let unit_start = pos;
					while pos < line.len() && line[pos].is_ascii_alphabetic() {
						pos += 1;
					}
					Some(unsafe { ::std::str::from_utf8_unchecked(&line[unit_start..pos]) })
				} else {
					None
				};

				Some((key, value, unit))
			}
		},
		line_pattern: quote! { (key, value, unit) },
	};

	generate_line_based_parser(
		s,
		field_inits,
		error_name,
		config,
		|s| generate_kv_phf_lookup(s, error_name),
		|s| generate_kv_match_arms(s, error_name),
	)
}

/// Build PHF map from field keys using phf_map! macro
/// Returns the map definition to be placed inside the parse function
fn build_phf_map(keys: &[(usize, String)]) -> TokenStream {
	let entries: Vec<_> = keys
		.iter()
		.map(|(idx, key)| {
			quote! { #key => #idx }
		})
		.collect();

	// Use a local static inside the function to avoid name collisions
	quote! {
		static FIELD_MAP: ::phf::Map<&'static str, usize> = ::phf::phf_map! {
			#(#entries),*
		};
	}
}

/// Generate KV field parse expression (shared by PHF and match)
fn generate_kv_field_assignment(field: &Field, error_name: &syn::Ident) -> TokenStream {
	let name = &field.ident;
	let span = field.ident.span();
	let field_name_str = name.to_string();

	let parse_expr = if let Some(parser) = &field.parser {
		quote! {
			#parser(value, unit.as_deref()).map_err(|_| #error_name::ParseField {
				field: #field_name_str,
				value: value.to_string(),
				expected: "custom format",
			})?
		}
	} else if let Some(uom_code) = &field.uom_conversion_code {
		quote! { { #uom_code } }
	} else {
		let ty = if field.is_option_type { field.inner_type.as_ref().unwrap() } else { &field.ty };
		quote! {
			value.parse::<#ty>().map_err(|_| #error_name::ParseField {
				field: #field_name_str,
				value: value.to_string(),
				expected: stringify!(#ty),
			})?
		}
	};

	if field.is_option_type {
		quote_spanned! { span =>
			result.#name = Some(#parse_expr);
			matched += 1;
		}
	} else {
		quote_spanned! { span =>
			result.#name = #parse_expr;
			matched += 1;
		}
	}
}

/// Generate PHF map and field handlers (shared by KV and Space)
fn generate_phf_lookup<F>(
	s: &Struct,
	key_fn: impl Fn(&Field) -> String,
	arm_fn: F,
) -> (TokenStream, TokenStream)
where
	F: Fn(usize, &Field) -> TokenStream,
{
	let fields_info: Vec<_> = s
		.fields
		.iter()
		.enumerate()
		.map(|(idx, field)| {
			let key = key_fn(field);
			(idx, key, field)
		})
		.collect();

	let keys: Vec<_> = fields_info.iter().map(|(idx, key, _)| (*idx, key.clone())).collect();
	let phf_map = build_phf_map(&keys);

	let field_arms: Vec<_> = fields_info
		.iter()
		.map(|(idx, _, field)| {
			let arm = arm_fn(*idx, field);
			let span = field.ident.span();
			quote_spanned! { span => #idx => { #arm } }
		})
		.collect();

	let handlers = quote! {
		match field_id {
			#(#field_arms)*
			_ => {}
		}
	};

	(phf_map, handlers)
}

/// Generate PHF map and field handlers for KV format
fn generate_kv_phf_lookup(s: &Struct, error_name: &syn::Ident) -> (TokenStream, TokenStream) {
	generate_phf_lookup(
		s,
		|field| field.effective_key(),
		|_, field| generate_kv_field_assignment(field, error_name),
	)
}

fn generate_kv_match_arms(s: &Struct, error_name: &syn::Ident) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let span = field.ident.span();
			let key = field.effective_key();
			let assignment = generate_kv_field_assignment(field, error_name);

			quote_spanned! { span =>
				#key => { #assignment }
			}
		})
		.collect()
}

/// Space Format Parser with Perfect Hashing for large structs
fn generate_space_parser(
	s: &Struct,
	field_inits: &[TokenStream],
	error_name: &syn::Ident,
) -> (TokenStream, TokenStream) {
	// Generate SIMD or fallback whitespace finding
	let find_whitespace = if use_simd() {
		quote! { ::memchr::memchr2(b' ', b'\t', line)? }
	} else {
		quote! { line.iter().position(|&b| b == b' ' || b == b'\t')? }
	};

	let config = LineParserConfig {
		line_parser_fn: quote! {
			/// Fast space-separated line parser - extracts key and u64 value
			#[inline]
			fn parse_line(line: &[u8]) -> Option<(&str, u64)> {
				let ws_pos = #find_whitespace;
				let key = unsafe { ::std::str::from_utf8_unchecked(&line[..ws_pos]) };

				let mut pos = ws_pos + 1;
				while pos < line.len() && (line[pos] == b' ' || line[pos] == b'\t') {
					pos += 1;
				}

				let mut value: u64 = 0;
				while pos < line.len() && line[pos].is_ascii_digit() {
					value = value.wrapping_mul(10).wrapping_add((line[pos] - b'0') as u64);
					pos += 1;
				}

				Some((key, value))
			}
		},
		line_pattern: quote! { (key, value) },
	};

	generate_line_based_parser(
		s,
		field_inits,
		error_name,
		config,
		generate_space_phf_lookup,
		generate_space_match_arms,
	)
}

/// Generate PHF map and field handlers for Space format
fn generate_space_phf_lookup(s: &Struct) -> (TokenStream, TokenStream) {
	generate_phf_lookup(
		s,
		|field| field.key.clone().unwrap_or_else(|| field.ident.to_string()),
		|_, field| generate_space_field_assignment(field),
	)
}

/// Generate Space field assignment (shared by PHF and match)
fn generate_space_field_assignment(field: &Field) -> TokenStream {
	let name = &field.ident;
	let span = field.ident.span();

	if field.is_option_type {
		quote_spanned! { span =>
			result.#name = Some(value);
			matched += 1;
		}
	} else {
		quote_spanned! { span =>
			result.#name = value;
			matched += 1;
		}
	}
}

fn generate_space_match_arms(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let name = &field.ident;
			let span = field.ident.span();
			let key = field.key.clone().unwrap_or_else(|| name.to_string());
			let assignment = generate_space_field_assignment(field);

			quote_spanned! { span =>
				#key => { #assignment }
			}
		})
		.collect()
}

/// Table Format Parser
fn generate_table_parser(
	s: &Struct,
	field_inits: &[TokenStream],
	error_name: &syn::Ident,
) -> TokenStream {
	let parse_arms = generate_table_arms(s);
	// Calculate max field index needed
	let max_fields = s
		.fields
		.iter()
		.enumerate()
		.map(|(i, f)| f.index.unwrap_or(i))
		.max()
		.unwrap_or(0) +
		1;

	// Reuse the same LineIter as KV/Space parsers
	let line_iter = generate_line_iterator();

	// Generate SIMD or fallback whitespace finding for split_fields
	let find_whitespace_in_split = if use_simd() {
		quote! { ::memchr::memchr2(b' ', b'\t', &bytes[pos..]) }
	} else {
		quote! { bytes[pos..].iter().position(|&b| b == b' ' || b == b'\t') }
	};

	quote! {
		/// Parse all records from /proc file content.
		pub fn parse_all(input: &str) -> Result<Vec<Self>, #error_name> {
			Self::parse_iter(input).collect()
		}

		/// Zero-copy iterator over records with line splitting.
		#[inline]
		pub fn parse_iter(input: &str) -> impl Iterator<Item = Result<Self, #error_name>> + '_ {
			#line_iter

			LineIter::new(input).filter_map(|line| {
				// Skip empty lines and comments
				if line.is_empty() || line[0] == b'#' {
					return None;
				}
				// Convert to str for parsing
				let line_str = unsafe { ::std::str::from_utf8_unchecked(line) };
				Some(Self::parse_line(line_str))
			})
		}

		/// Parse a single line into a record.
		#[inline]
		fn parse_line(line: &str) -> Result<Self, #error_name> {
			let mut fields: [&str; #max_fields] = [""; #max_fields];
			let count = Self::split_fields(line, &mut fields);
			if count == 0 {
				return Err(#error_name::InvalidFormat {
					line: 0,
					content: line.to_string(),
				});
			}

			let mut result = Self { #(#field_inits,)* };
			#(#parse_arms)*
			Ok(result)
		}

		/// Fast u64 parsing without allocation or error handling overhead
		#[inline]
		fn parse_u64_fast(s: &str) -> Option<u64> {
			let bytes = s.as_bytes();
			if bytes.is_empty() {
				return None;
			}
			let mut value: u64 = 0;
			for &b in bytes {
				if !b.is_ascii_digit() {
					return None;
				}
				value = value.wrapping_mul(10).wrapping_add((b - b'0') as u64);
			}
			Some(value)
		}

		/// Field splitting with optional SIMD acceleration.
		#[inline]
		fn split_fields<'a>(line: &'a str, out: &mut [&'a str; #max_fields]) -> usize {
			let bytes = line.as_bytes();
			let len = bytes.len();
			let mut count = 0;
			let mut pos = 0;

			// Skip leading whitespace
			while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
				pos += 1;
			}

			while pos < len && count < #max_fields {
				let field_start = pos;

				// Find next whitespace (space or tab)
				if let Some(ws_offset) = #find_whitespace_in_split {
					let field_end = pos + ws_offset;
					out[count] = unsafe { ::std::str::from_utf8_unchecked(&bytes[field_start..field_end]) };
					count += 1;
					pos = field_end + 1;

					// Skip consecutive whitespace
					while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
						pos += 1;
					}
				} else {
					// No more whitespace, rest is last field
					out[count] = unsafe { ::std::str::from_utf8_unchecked(&bytes[field_start..]) };
					count += 1;
					break;
				}
			}

			count
		}
	}
}

fn generate_table_arms(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.enumerate()
		.map(|(i, field)| {
			let name = &field.ident;
			let span = field.ident.span();
			let index = field.index.unwrap_or(i);

			// Use direct array indexing since we know MAX_FIELDS >= index
			if let Some(parser) = &field.parser {
				let assign = if field.is_option_type {
					quote! { result.#name = Some(#parser(parsed_value)); }
				} else {
					quote! { result.#name = #parser(parsed_value); }
				};
				// Use fast u64 parsing for custom parser fields
				quote_spanned! { span =>
					if #index < count {
						if let Some(parsed_value) = Self::parse_u64_fast(fields[#index]) {
							#assign
						}
					}
				}
			} else if let Some(uom_code) = &field.uom_conversion_code {
				let adapted = quote_spanned! { span => { let value = field_value; #uom_code } };
				let assign = if field.is_option_type {
					quote! { result.#name = Some(#adapted); }
				} else {
					quote! { result.#name = #adapted; }
				};
				quote_spanned! { span =>
					if #index < count {
						let field_value = fields[#index];
						if !field_value.is_empty() {
							#assign
						}
					}
				}
			} else {
				let ty =
					if field.is_option_type { field.inner_type.as_ref().unwrap() } else { &field.ty };
				// Check if type is u64 to use fast parsing
				let type_str = quote::quote!(#ty).to_string();
				let assign = if field.is_option_type {
					quote! { result.#name = Some(parsed_value); }
				} else {
					quote! { result.#name = parsed_value; }
				};
				if type_str == "u64" {
					quote_spanned! { span =>
						if #index < count {
							if let Some(parsed_value) = Self::parse_u64_fast(fields[#index]) {
								#assign
							}
						}
					}
				} else {
					quote_spanned! { span =>
						if #index < count {
							let field_value = fields[#index];
							if !field_value.is_empty() {
								if let Ok(parsed_value) = field_value.parse::<#ty>() {
									#assign
								}
							}
						}
					}
				}
			}
		})
		.collect()
}
