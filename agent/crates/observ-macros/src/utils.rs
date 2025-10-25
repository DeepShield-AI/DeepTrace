use heck::ToPascalCase;
use syn::{GenericArgument, Ident, PathArguments, Type};
/// Extract the inner type from Option<T>, returns None if not an Option type
pub fn extract_option_inner_type(ty: &Type) -> Option<Type> {
	let path = match ty {
		Type::Path(ty) => &ty.path,
		_ => return None,
	};

	let last = path.segments.last().unwrap();
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

/// Check if a type string contains a specific UOM type name
pub fn contains_uom_type(type_string: &str, uom_name: &str) -> bool {
	let ty = type_string.replace(" ", "");
	ty.contains(uom_name)
}

/// Convert a type to a clean string representation for analysis
pub fn type_to_string(ty: &Type) -> String {
	quote::quote! { #ty }.to_string()
}

/// Canonicalize a Rust field name to its corresponding /proc file key format
///
/// Examples:
/// - `mem_total` → `MemTotal`
/// - `active_anon` → `Active(anon)`
/// - `inactive_file` → `Inactive(file)`
pub fn canonicalize_field_name(field_name: &Ident) -> String {
	let name = field_name.to_string();

	// Handle snake_case with underscores that should become parentheses
	// Pattern: prefix_suffix -> Prefix(suffix)
	if let Some((prefix, suffix)) = name.split_once('_') &&
		should_use_parenthetical_format(prefix, suffix)
	{
		return format!("{}({})", prefix.to_pascal_case(), suffix);
	}

	// For regular names, convert snake_case to PascalCase
	name.to_pascal_case()
}

/// Determine if a prefix_suffix pattern should use parenthetical format: Prefix(suffix)
/// Based on common /proc file naming conventions
fn should_use_parenthetical_format(prefix: &str, suffix: &str) -> bool {
	const PARENTHETICAL_PREFIXES: &[&str] = &["active", "inactive", "slab", "kernel"];
	const PARENTHETICAL_SUFFIXES: &[&str] =
		&["anon", "file", "reclaimable", "unreclaimable", "stack"];

	PARENTHETICAL_PREFIXES.contains(&prefix.to_lowercase().as_str()) &&
		PARENTHETICAL_SUFFIXES.contains(&suffix.to_lowercase().as_str())
}
