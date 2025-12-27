//! Validation for ProcParser derive macro.
//!
//! Validates attribute combinations and field configurations before code generation.

use super::{
	ast::{Field, Struct},
	attr::FormatKind,
	utils,
};
use std::collections::HashSet;

impl Struct {
	/// Validate the struct configuration
	pub fn validate(&self) -> darling::Result<()> {
		let mut errors = darling::Error::accumulator();

		// Validate all fields
		for field in &self.fields {
			errors.handle(field.validate(&self.format));
		}

		// Check for duplicates based on format
		match self.format {
			FormatKind::Table => { errors.handle(self.check_duplicate_indices()); },
			_ => { errors.handle(self.check_duplicate_keys()); },
		}

		errors.finish()
	}

	fn check_duplicate_keys(&self) -> darling::Result<()> {
		let mut seen = HashSet::new();

		for field in &self.fields {
			let key = field.effective_key();
			if !seen.insert(key.clone()) {
				return Err(darling::Error::custom(format!("duplicate key `{key}`"))
					.with_span(&field.ident));
			}
		}

		Ok(())
	}

	fn check_duplicate_indices(&self) -> darling::Result<()> {
		let mut seen = HashSet::new();

		for (i, field) in self.fields.iter().enumerate() {
			let index = field.index.unwrap_or(i);
			if !seen.insert(index) {
				return Err(darling::Error::custom(format!("duplicate index `{index}`"))
					.with_span(&field.ident));
			}
		}

		Ok(())
	}
}

impl Field {
	fn validate(&self, format: &FormatKind) -> darling::Result<()> {
		// index attribute only valid with table format
		if self.index.is_some() && !matches!(format, FormatKind::Table) {
			return Err(darling::Error::custom(
				"`index` attribute requires `#[fmt = \"table\"]`",
			)
			.with_span(&self.ident));
		}

		// key attribute not valid with table format
		if self.key.is_some() && matches!(format, FormatKind::Table) {
			return Err(darling::Error::custom(
				"use `index` instead of `key` for table format",
			)
			.with_span(&self.ident));
		}

		// optional attribute requires Option<T> type
		if self.optional && !self.is_option_type {
			return Err(
				darling::Error::custom("`optional` attribute requires `Option<T>` type")
					.with_span(&self.ident),
			);
		}

		Ok(())
	}

	/// Get the effective key for this field (custom key or derived from field name)
	pub fn effective_key(&self) -> String {
		self.key
			.clone()
			.unwrap_or_else(|| utils::canonicalize_field_name(&self.ident))
	}
}
