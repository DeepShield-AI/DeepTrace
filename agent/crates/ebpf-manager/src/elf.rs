use crate::Result;
use object::{Object, ObjectSection, ObjectSymbol};
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone)]
pub struct Symbol {
	pub section_name: String,
}

#[derive(Debug, Default)]
pub struct Elf {
	symbols: FxHashMap<String, Symbol>,
}

impl Elf {
	pub fn from_raw_elf(data: &[u8]) -> Result<Self> {
		let obj = object::read::File::parse(data)?;
		let mut s: Self = Default::default();

		for sym in obj.symbols() {
			if let Some(section) = sym.section_index().and_then(|i| obj.section_by_index(i).ok()) &&
				let (Ok(sym_name), Ok(sec_name)) = (sym.name(), section.name())
			{
				s.symbols
					.insert(sym_name.to_string(), Symbol { section_name: sec_name.to_string() });
			}
		}
		Ok(s)
	}

	pub fn get_by_symbol_name<S: AsRef<str>>(&self, sym_name: S) -> Option<&Symbol> {
		self.symbols.get(sym_name.as_ref())
	}
}
