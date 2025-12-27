//! Common AST structures shared across macros.

use syn::Ident;

/// Represents a field member - either named or unnamed (tuple struct)
#[derive(Clone)]
pub enum MemberUnraw {
	Named(Ident),
	Unnamed,
}
