//! Common utility functions shared across all macros.

use syn::{GenericArgument, PathArguments, Type};

/// Extract the inner type from Option<T>, returns None if not an Option type
pub fn extract_option_inner_type(ty: &Type) -> Option<Type> {
	let path = match ty {
		Type::Path(ty) => &ty.path,
		_ => return None,
	};

	let last = path.segments.last()?;
	if last.ident != "Option" {
		return None;
	}

	let args = match &last.arguments {
		PathArguments::AngleBracketed(args) => args,
		_ => return None,
	};

	if let Some(GenericArgument::Type(inner)) = args.args.first() {
		Some(inner.clone())
	} else {
		None
	}
}

/// Convert a type to a clean string representation for analysis
pub fn type_to_string(ty: &Type) -> String {
	quote::quote! { #ty }.to_string()
}

/// Check if a type string contains a specific pattern
pub fn type_contains(type_string: &str, pattern: &str) -> bool {
	type_string.replace(' ', "").contains(pattern)
}
