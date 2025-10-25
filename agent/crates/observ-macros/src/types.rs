use crate::utils;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, Type};

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

pub fn generate_uom_conversion_code_with_unit(
	ty: &Type,
	unit: Option<&Path>,
) -> Option<TokenStream> {
	let type_string = utils::type_to_string(ty);

	let (_, type_name, default_unit) = UOM_TYPES
		.iter()
		.find(|(pattern, ..)| utils::contains_uom_type(&type_string, pattern))?;

	let unit_path = if let Some(u) = unit {
		quote! { #u }
	} else {
		let default: TokenStream = default_unit.parse().ok()?;
		default
	};

	let type_ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());
	let (constructor, cast) = match () {
		_ if type_string.contains("u64::") =>
			(quote! { uom::si::u64::#type_ident::new::<#unit_path> }, quote! { as u64 }),
		_ if type_string.contains("f32::") =>
			(quote! { uom::si::f32::#type_ident::new::<#unit_path> }, quote! { as f32 }),
		_ if type_string.contains("f64::") =>
			(quote! { uom::si::f64::#type_ident::new::<#unit_path> }, quote! {}),
		_ => (quote! { #ty::new::<#unit_path> }, quote! {}),
	};

	Some(quote! {
		{
			let parsed_value: f64 = value.parse().map_err(|_| "Failed to parse value")?;
			#constructor(parsed_value #cast)
		}
	})
}
