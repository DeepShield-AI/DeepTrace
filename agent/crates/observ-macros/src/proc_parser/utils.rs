//! Utility functions specific to ProcParser.

use heck::ToPascalCase;
use syn::Ident;

/// Canonicalize a Rust field name to its corresponding /proc file key format.
///
/// # Examples
/// - `mem_total` → `MemTotal`
/// - `active_anon` → `Active(anon)`
/// - `inactive_file` → `Inactive(file)`
pub fn canonicalize_field_name(field_name: &Ident) -> String {
	let name = field_name.to_string();

	// Handle special parenthetical format for certain /proc fields
	if let Some((prefix, suffix)) = name.split_once('_') {
		if should_use_parenthetical_format(prefix, suffix) {
			return format!("{}({})", prefix.to_pascal_case(), suffix);
		}
	}

	// Default: convert snake_case to PascalCase
	name.to_pascal_case()
}

/// Determine if a prefix_suffix pattern should use parenthetical format.
/// Based on common /proc file naming conventions.
fn should_use_parenthetical_format(prefix: &str, suffix: &str) -> bool {
	const PARENTHETICAL_PREFIXES: &[&str] = &["active", "inactive", "slab", "kernel"];
	const PARENTHETICAL_SUFFIXES: &[&str] =
		&["anon", "file", "reclaimable", "unreclaimable", "stack"];

	PARENTHETICAL_PREFIXES.contains(&prefix.to_lowercase().as_str()) &&
		PARENTHETICAL_SUFFIXES.contains(&suffix.to_lowercase().as_str())
}
