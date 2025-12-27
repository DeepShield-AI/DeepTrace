//! Test: Typo in format value should suggest correct value.

use observ_macros::ProcParser;

#[derive(ProcParser)]
#[fmt = "keyvalue"]
struct Test {
	field: u64,
}

fn main() {}
