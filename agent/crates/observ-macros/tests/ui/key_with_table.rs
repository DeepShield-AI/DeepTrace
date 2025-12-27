//! Test: key attribute not valid with table format.

use observ_macros::ProcParser;

#[derive(ProcParser)]
#[fmt = "table"]
struct Test {
	#[arg(key = "test")]
	field: u64,
}

fn main() {}
