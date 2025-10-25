use crate::{
	ast::{Field, Struct},
	attr::structs::FormatKind,
	utils,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub(crate) fn derive(input: &syn::DeriveInput) -> Result<TokenStream> {
	try_expand(input)
}

fn try_expand(input: &syn::DeriveInput) -> Result<TokenStream> {
	let s = Struct::from_syn(input)?;
	Ok(impl_struct(&s))
}

fn impl_struct(s: &Struct) -> TokenStream {
	let struct_name = &s.ident;
	let format = s.attrs.format.as_ref().unwrap_or(&FormatKind::Kv);

	let parse_impl = generate_parse_impl(s, *format);
	let getters = generate_field_getters(&s.fields, Some(s.visibility));
	let helper_functions = generate_conversion_helpers();

	quote! {
		impl #struct_name {
			#parse_impl
			#(#getters)*
			#helper_functions
		}
	}
}

fn generate_parse_impl(s: &Struct, format: FormatKind) -> TokenStream {
	let field_inits = generate_field_inits(s);

	match format {
		FormatKind::Kv => generate_kv_parser(s, &field_inits),
		FormatKind::Space => generate_space_parser(s, &field_inits),
		FormatKind::Table => generate_table_parser(s, &field_inits),
	}
}

fn generate_field_inits(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let field_name = match &field.member {
				crate::ast::MemberUnraw::Named(ident) => &ident.raw,
				_ => panic!("Only named fields are supported"),
			};

			if field.is_optional {
				quote! { #field_name: None }
			} else {
				quote! { #field_name: Default::default() }
			}
		})
		.collect()
}

fn generate_kv_parser(s: &Struct, field_inits: &[TokenStream]) -> TokenStream {
	let parse_arms = generate_kv_parse_arms(s);

	quote! {
		pub fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
			use nom::{
				bytes::complete::take_while1,
				character::complete::{alpha1, char, digit1, multispace0, space0},
				combinator::{opt, recognize},
				sequence::{pair, terminated},
				IResult, Parser,
			};

			fn parse_kv_line(input: &str) -> Result<(&str, &str, Option<&str>), Box<dyn std::error::Error>> {
				fn parse_line(input: &str) -> IResult<&str, (&str, &str, Option<&str>)> {
					// Parse key: alphanumeric with underscores, parentheses
					let (input, key) = terminated(
						take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '(' || c == ')'),
						char(':'),
					).parse(input)?;

					// Skip whitespace after colon
					let (input, _) = multispace0(input)?;

					// Parse numeric value (integer or float)
					let (input, value) = recognize(pair(
						digit1,
						opt(pair(char('.'), digit1))
					)).parse(input)?;

					// Skip whitespace before unit
					let (input, _) = space0(input)?;

					// Parse optional unit (letters only)
					let (input, unit) = opt(alpha1).parse(input)?;

					Ok((input, (key, value, unit)))
				}

				parse_line(input.trim())
					.map(|(_, result)| result)
					.map_err(|e| format!("Nom parse error: {}", e).into())
			}

			let mut result = Self {
				#(#field_inits,)*
			};

			for line in input.lines() {
				let line = line.trim();
				if line.is_empty() || line.starts_with('#') {
					continue;
				}

				// Use nom-based parsing directly
				if let Ok((key, value, unit)) = parse_kv_line(line) {
					#(#parse_arms)*
				}
			}

			Ok(result)
		}
	}
}

fn generate_space_parser(s: &Struct, field_inits: &[TokenStream]) -> TokenStream {
	let parse_arms = generate_space_parse_arms(s);

	quote! {
		pub fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
			use nom::{
				character::complete::{alphanumeric1, digit1, multispace1, space1},
				combinator::{map_res, recognize},
				multi::separated_list0,
				sequence::pair,
				IResult, Parser,
			};

			fn parse_space_line(input: &str) -> Result<(&str, u64), Box<dyn std::error::Error>> {
				fn parse_line(input: &str) -> IResult<&str, (&str, u64)> {
					use nom::bytes::complete::take_while1;

					// Parse key (alphanumeric identifier with underscores)
					let (input, key) = take_while1(|c: char| c.is_alphanumeric() || c == '_').parse(input)?;

					// Skip whitespace
					let (input, _) = multispace1(input)?;

					// Parse single numeric value
					let (input, value) = map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)?;

					Ok((input, (key, value)))
				}

				parse_line(input.trim())
					.map(|(_, result)| result)
					.map_err(|e| format!("Nom parse error: {}", e).into())
			}

			let mut result = Self {
				#(#field_inits,)*
			};

			for line in input.lines() {
				let line = line.trim();
				if line.is_empty() || line.starts_with('#') {
					continue;
				}

				// Use nom-based parsing directly
				if let Ok((key, value)) = parse_space_line(line) {
					#(#parse_arms)*
				}
			}

			Ok(result)
		}
	}
}

fn generate_table_parser(s: &Struct, field_inits: &[TokenStream]) -> TokenStream {
	let parse_arms = generate_table_parse_arms(s);

	quote! {
		pub fn parse_all(input: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
			use nom::{
				bytes::complete::take_while1,
				character::complete::{digit1, multispace1, space0},
				combinator::{map_res, opt, recognize},
				multi::separated_list0,
				sequence::pair,
				IResult, Parser,
			};

			fn parse_table_line(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
				fn parse_line(input: &str) -> IResult<&str, Vec<String>> {
					// Parse space-separated fields
					separated_list0(
						multispace1,
						map_res(
							take_while1(|c: char| !c.is_whitespace()),
							|s: &str| -> Result<String, std::convert::Infallible> { Ok(s.to_string()) }
						)
					).parse(input)
				}

				parse_line(input.trim())
					.map(|(_, result)| result)
					.map_err(|e| format!("Nom parse error: {}", e).into())
			}

			let mut results = Vec::new();

			for line in input.lines() {
				let line = line.trim();
				if line.is_empty() || line.starts_with('#') {
					continue;
				}

				// Use nom-based parsing
				if let Ok(fields) = parse_table_line(line) {
					if fields.is_empty() {
						continue;
					}

					let mut result = Self {
						#(#field_inits,)*
					};

					// Parse fields by position using nom-parsed data
					#(#parse_arms)*

					results.push(result);
				}
			}

			Ok(results)
		}
	}
}

fn generate_table_parse_arms(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.enumerate()
		.map(|(field_index, field)| {
			let field_name = match &field.member {
				crate::ast::MemberUnraw::Named(ident) => &ident.raw,
				_ => panic!("Only named fields are supported"),
			};

			// Use the index attribute if specified, otherwise use field position (0-based)
			// The index attribute is already 0-based, so use it directly
			let index = if let Some(attr_index) = field.attrs.index {
				attr_index // Use the index as-is (already 0-based)
			} else {
				field_index
			};

			let parser_path = field.attrs.parser.as_ref();

			if let Some(parser_path) = parser_path {
				if field.is_optional {
					quote! {
						if let Some(field_value) = fields.get(#index) {
							if let Ok(parsed_value) = field_value.parse::<u64>() {
								result.#field_name = Some(#parser_path(parsed_value));
							}
						}
					}
				} else {
					quote! {
						if let Some(field_value) = fields.get(#index) {
							if let Ok(parsed_value) = field_value.parse::<u64>() {
								result.#field_name = #parser_path(parsed_value);
							}
						}
					}
				}
			} else if let Some(uom_code) = &field.uom_conversion_code {
				// For UOM types in table format, we need to adapt the generated code
				// The UOM code expects 'value' but we have 'field_value'
				let adapted_uom_code = quote! {
					{
						let value = field_value.as_str();
						#uom_code
					}
				};

				if field.is_optional {
					quote! {
						if let Some(field_value) = fields.get(#index) {
							result.#field_name = Some(#adapted_uom_code);
						}
					}
				} else {
					quote! {
						if let Some(field_value) = fields.get(#index) {
							result.#field_name = #adapted_uom_code;
						}
					}
				}
			} else if field.is_optional {
				let ty = field.inner_type.as_ref().unwrap();
				quote! {
					if let Some(field_value) = fields.get(#index) {
						if let Ok(parsed_value) = field_value.parse::<#ty>() {
							result.#field_name = Some(parsed_value);
						}
					}
				}
			} else {
				let ty = field.ty;
				quote! {
					if let Some(field_value) = fields.get(#index) {
						if let Ok(parsed_value) = field_value.parse::<#ty>() {
							result.#field_name = parsed_value;
						}
					}
				}
			}
		})
		.collect()
}

fn generate_space_parse_arms(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let field_name = match &field.member {
				crate::ast::MemberUnraw::Named(ident) => &ident.raw,
				_ => panic!("Only named fields are supported"),
			};

			// For space format, use the exact field name (snake_case) instead of canonicalizing
			let key = field.attrs.key.as_ref().cloned().unwrap_or_else(|| field_name.to_string());

			if field.is_optional {
				quote! {
					if key == #key {
						result.#field_name = Some(value);
						continue;
					}
				}
			} else {
				quote! {
					if key == #key {
						result.#field_name = value;
						continue;
					}
				}
			}
		})
		.collect()
}

fn generate_kv_parse_arms(s: &Struct) -> Vec<TokenStream> {
	s.fields
		.iter()
		.map(|field| {
			let field_name = match &field.member {
				crate::ast::MemberUnraw::Named(ident) => &ident.raw,
				_ => panic!("Only named fields are supported"),
			};

			let key = field
				.attrs
				.key
				.as_ref()
				.cloned()
				.unwrap_or_else(|| utils::canonicalize_field_name(field_name));

			let parser_path = field.attrs.parser.as_ref();

			if let Some(parser_path) = parser_path {
				quote! {
					if key == #key {
						result.#field_name = #parser_path(value, unit.as_deref())?;
						continue;
					}
				}
			} else if let Some(uom_code) = &field.uom_conversion_code {
				generate_uom_parse_arm(field, &key, uom_code)
			} else {
				generate_default_parse_arm(field, &key)
			}
		})
		.collect()
}

fn generate_parse_arm(field: &Field, key: &str, value_expr: TokenStream) -> TokenStream {
	let field_name = match &field.member {
		crate::ast::MemberUnraw::Named(ident) => &ident.raw,
		_ => panic!("Only named fields are supported"),
	};

	let assignment = if field.is_optional {
		quote! { result.#field_name = Some(#value_expr); }
	} else {
		quote! { result.#field_name = #value_expr; }
	};

	quote! {
		if key == #key {
			#assignment
			continue;
		}
	}
}

fn generate_uom_parse_arm(field: &Field, key: &str, uom_code: &TokenStream) -> TokenStream {
	generate_parse_arm(field, key, uom_code.clone())
}

fn generate_default_parse_arm(field: &Field, key: &str) -> TokenStream {
	let ty = if field.is_optional { field.inner_type.as_ref().unwrap() } else { field.ty };
	let parse_expr = quote! { value.to_string().parse::<#ty>()? };
	generate_parse_arm(field, key, parse_expr)
}

fn generate_field_getters(
	fields: &[Field],
	struct_visibility: Option<&syn::Visibility>,
) -> Vec<TokenStream> {
	fields
		.iter()
		.map(|field| {
			let field_name = match &field.member {
				crate::ast::MemberUnraw::Named(ident) => &ident.raw,
				_ => panic!("Only named fields are supported"),
			};

			let ty = field.ty;

			// Use field visibility if it's more restrictive than private,
			// otherwise use struct visibility as fallback
			let vis = match field.visibility {
				syn::Visibility::Inherited => {
					// Field has no explicit visibility, use struct visibility
					struct_visibility.map(|v| quote! { #v }).unwrap_or_else(|| quote! { pub })
				},
				field_vis => {
					// Field has explicit visibility, use it
					quote! { #field_vis }
				},
			};

			quote! {
				paste::paste! {
					#vis fn [<get_ #field_name>](&self) -> &#ty {
						&self.#field_name
					}
				}
			}
		})
		.collect()
}

/// Generate UOM conversion helper functions
fn generate_conversion_helpers() -> TokenStream {
	quote! {
		// UOM conversion helper function for basic unit conversions
		fn convert_with_unit(value: &str, unit: &str) -> Result<f64, Box<dyn std::error::Error>> {
			let parsed_value: f64 = value.parse()?;

			// Basic unit conversion - can be extended for more units
			match unit.to_lowercase().as_str() {
				"kb" => Ok(parsed_value * 1024.0),
				"mb" => Ok(parsed_value * 1024.0 * 1024.0),
				"gb" => Ok(parsed_value * 1024.0 * 1024.0 * 1024.0),
				"b" | "bytes" => Ok(parsed_value),
				_ => Ok(parsed_value), // Default: no conversion
			}
		}
	}
}
