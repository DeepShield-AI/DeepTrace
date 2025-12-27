//! UOM type handling for ProcParser.

use crate::common;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, Type};

/// Supported UOM types with their default units
const UOM_TYPES: &[(&str, &str, &str)] = &[
	("Information", "Information", "uom::si::information::byte"),
	("Time", "Time", "uom::si::time::second"),
	("Frequency", "Frequency", "uom::si::frequency::hertz"),
	("Length", "Length", "uom::si::length::meter"),
	("Mass", "Mass", "uom::si::mass::gram"),
	("Ratio", "Ratio", "uom::si::ratio::ratio"),
	(
		"ThermodynamicTemperature",
		"ThermodynamicTemperature",
		"uom::si::thermodynamic_temperature::kelvin",
	),
];

/// Generate UOM conversion code for a type with optional custom unit.
pub fn generate_uom_conversion(ty: &Type, unit: Option<&Path>) -> Option<TokenStream> {
	let type_ = common::type_to_string(ty);

	let (_, type_name, default_unit) = UOM_TYPES
		.iter()
		.find(|(pattern, ..)| common::type_contains(&type_, pattern))?;

	let unit = if let Some(u) = unit {
		quote! { #u }
	} else {
		default_unit.parse().ok()?
	};

	let type_ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());

	let (constructor, cast) = if type_.contains("u64::") {
		(quote! { uom::si::u64::#type_ident::new::<#unit> }, quote! { as u64 })
	} else if type_.contains("f32::") {
		(quote! { uom::si::f32::#type_ident::new::<#unit> }, quote! { as f32 })
	} else if type_.contains("f64::") {
		(quote! { uom::si::f64::#type_ident::new::<#unit> }, quote! {})
	} else {
		(quote! { #ty::new::<#unit> }, quote! {})
	};

	Some(quote! {
		{
			let parsed_value: f64 = value.parse().map_err(|_| "Failed to parse value")?;
			#constructor(parsed_value #cast)
		}
	})
}
