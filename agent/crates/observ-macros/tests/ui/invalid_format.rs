//! Test: Invalid format value should produce helpful error message.

use observ_macros::ProcParser;

#[derive(ProcParser)]
#[fmt = "invalid"]
struct Test {
	field: u64,
}

fn main() {}
