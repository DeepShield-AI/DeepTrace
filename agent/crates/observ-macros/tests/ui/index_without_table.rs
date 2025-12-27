//! Test: index attribute requires table format.

use observ_macros::ProcParser;

#[derive(ProcParser)]
#[fmt = "kv"]
struct Test {
	#[arg(index = 0)]
	field: u64,
}

fn main() {}
