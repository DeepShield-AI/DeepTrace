//! Test: optional attribute requires Option<T> type.

use observ_macros::ProcParser;

#[derive(ProcParser)]
struct Test {
	#[arg(optional)]
	field: u64,
}

fn main() {}
