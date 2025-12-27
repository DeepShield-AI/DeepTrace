//! UI tests for ProcParser derive macro.
//!
//! These tests verify that the macro produces helpful error messages
//! for common mistakes.

#[test]
fn ui_tests() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/*.rs");
}
