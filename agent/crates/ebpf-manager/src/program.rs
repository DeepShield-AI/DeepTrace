use crate::{
	EbpfError, Result,
	elf::{Elf, Symbol},
	link::LinkId,
	version::{max_version, min_version},
};
use aya::{
	Btf, Ebpf,
	programs::{self, ProgramType},
	util::KernelVersion,
};
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct Programs<'a> {
	inner: FxHashMap<&'a str, Program<'a>>,
}

impl<'a> Programs<'a> {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn with_ebpf(ebpf: &'a mut Ebpf) -> Self {
		let inner = ebpf
			.programs_mut()
			.map(|(name, p)| {
				let prog = Program::new(name.to_string(), p);
				(name, prog)
			})
			.collect();

		Self { inner }
	}

	pub fn with_elf_info(mut self, data: &[u8]) -> Result<Self> {
		let elf_info = Elf::from_raw_elf(data)?;
		// prog_name is an Elf symbol name
		for (prog_name, prog) in self.inner.iter_mut() {
			if let Some(sym_info) = elf_info.get_by_symbol_name(prog_name) {
				prog.with_sym_info(sym_info.clone());
			}
		}
		Ok(self)
	}

	pub fn program_mut<S: AsRef<str>>(&mut self, name: S) -> &mut Program<'a> {
		self.inner
			.get_mut(name.as_ref())
			.unwrap_or_else(|| panic!("missing probe {}", name.as_ref()))
	}

	pub fn sorted_by_prio(&mut self) -> Vec<(&&str, &mut Program<'a>)> {
		let mut sorted = self.inner.iter_mut().collect::<Vec<_>>();
		sorted.sort_unstable_by_key(|(_, p)| (p.priority_by_program(), p.name.clone()));
		sorted
	}
}

pub struct Program<'a> {
	/// The name of the program.
	pub name: String,
	/// Whether the program is enabled.
	pub enable: bool,
	/// The priority of the program.
	pub priority: u8,
	/// The minimum kernel version required for the program.
	min_version: Option<KernelVersion>,
	/// The maximum kernel version required for the program.
	max_version: Option<KernelVersion>,
	/// The symbol information for the program.
	info: Option<Symbol>,
	program: &'a mut programs::Program,
	/// The attach point for the program.
	pub attach_point: Option<String>,
	/// Program link id used for detach program.
	link_id: Option<LinkId>,
	loaded: bool,
	attached: bool,
}

impl<'a> Program<'a> {
	const fn new(name: String, p: &'a mut programs::Program) -> Program<'a> {
		Program {
			name,
			enable: false,
			priority: 50,
			min_version: None,
			max_version: None,
			info: None,
			program: p,
			attach_point: None,
			link_id: None,
			loaded: false,
			attached: false,
		}
	}
}

/// Setter and Getter
impl<'a> Program<'a> {
	pub const fn set_min_kernel_version(&mut self, min_kernel_version: KernelVersion) -> &mut Self {
		self.min_version = Some(min_kernel_version);
		self
	}

	pub fn min_kernel_version(&self) -> &KernelVersion {
		self.min_version.as_ref().unwrap_or(min_version())
	}

	pub const fn set_max_kernel_version(&mut self, max_kernel_version: KernelVersion) -> &mut Self {
		self.max_version = Some(max_kernel_version);
		self
	}

	pub fn max_kernel_version(&self) -> &KernelVersion {
		self.max_version.as_ref().unwrap_or(max_version())
	}

	pub const fn set_priority(&mut self, priority: u8) -> &mut Self {
		self.priority = priority;
		self
	}

	pub fn prog_type(&self) -> ProgramType {
		self.program.prog_type()
	}

	pub const fn prog(&self) -> &programs::Program {
		self.program
	}

	pub const fn prog_mut(&mut self) -> &mut programs::Program {
		self.program
	}

	pub const fn enable(&mut self) -> &mut Self {
		self.enable = true;
		self
	}

	pub const fn disable(&mut self) -> &mut Self {
		self.enable = false;
		self
	}

	pub fn with_sym_info(&mut self, info: Symbol) -> &mut Self {
		self.info = Some(info);
		self.attach_point = self
			.info
			.as_ref()
			.and_then(|i| i.section_name.split('/').next_back().map(|s| s.to_string()));
		self
	}

	#[inline]
	fn tracepoint_category(&self) -> Option<String> {
		self.info.as_ref().and_then(|i| {
			let v: Vec<&str> = i.section_name.split('/').collect();
			v.get(v.len() - 2).map(|s| s.to_string())
		})
	}

	// naturally decrease priority of exit kind of probes to remove map operations errors at BPF load time
	fn priority_by_program(&self) -> u8 {
		let program = self.prog();

		match program {
			programs::Program::TracePoint(_) => {
				let kernel_attach = self
					.attach_point
					.as_ref()
					.ok_or(EbpfError::NoAttachFunction(self.name.clone()))
					.unwrap();
				if kernel_attach.starts_with("sys_exit") {
					return self.priority + 1;
				}
				self.priority
			},
			_ => self.priority,
		}
	}
}

/// Check logic
impl<'a> Program<'a> {
	/// Returns true if `with` [`KernelVersion`](aya::util::KernelVersion) is within range `[ self.min..self.max]`
	pub fn is_compatible(&self, with: &KernelVersion) -> bool {
		self.min_kernel_version() <= with && with <= self.max_kernel_version()
	}

	pub const fn disable_if(&mut self, condition: bool) -> &mut Self {
		if condition {
			self.enable = false
		}
		self
	}
	// TODO: is this necessary?
	/// Returns true if the attach point of program is `name`
	#[inline]
	pub fn has_attach_point<S: AsRef<str>>(&self, name: S) -> bool {
		self.attach_point
			.as_ref()
			.map(|a| a.as_str() == name.as_ref())
			.unwrap_or_default()
	}
}

/// Manage ebpf program lifecycle
impl<'a> Program<'a> {
	pub fn load(&mut self) -> Result<()> {
		let program = self.prog_mut();

		match program {
			programs::Program::TracePoint(p) => {
				p.load()?;
			},
			_ => {
				unimplemented!()
			},
		}
		self.loaded = true;
		Ok(())
	}

	pub fn load_with_btf(&mut self, btf: &Btf) -> Result<()> {
		// Get attach_point and name before borrowing program mutably
		let attach_point = self.attach_point.clone();
		let prog_name = self.name.clone();
		let program = self.prog_mut();

		match program {
			programs::Program::TracePoint(p) => {
				// TracePoint programs don't need BTF
				p.load()?;
			},
			programs::Program::Lsm(p) => {
				// LSM programs need BTF
				let attach_point = attach_point.ok_or(EbpfError::NoAttachFunction(prog_name))?;
				p.load(&attach_point, btf)?;
			},
			_ => {
				unimplemented!()
			},
		}
		self.loaded = true;
		Ok(())
	}

	pub fn attach(&mut self) -> Result<()> {
		let program_name = self.name.clone();
		let name = self
			.attach_point
			.clone()
			.ok_or(EbpfError::NoAttachFunction(program_name.clone()))?;
		let tracepoint_category = self.tracepoint_category();
		let program = self.prog_mut();

		match program {
			programs::Program::TracePoint(p) => {
				let category =
					tracepoint_category.ok_or(EbpfError::NoTracepointCategory(program_name))?;
				self.link_id = Some(LinkId::Tracepoint(p.attach(&category, &name)?));
			},
			_ => {
				unimplemented!()
			},
		}
		self.attached = true;
		Ok(())
	}

	pub fn load_and_attach(&mut self) -> Result<()> {
		self.load()?;
		self.attach()
	}

	pub fn load_and_attach_with_btf(&mut self, btf: &Btf) -> Result<()> {
		self.load_with_btf(btf)?;
		self.attach()
	}

	pub fn detach(&mut self) -> Result<()> {
		if let Some(link_id) = self.link_id.take() {
			match self.prog_mut() {
				programs::Program::TracePoint(p) => p.detach(link_id.try_into()?)?,
				_ => {
					unimplemented!()
				},
			}
		}
		self.attached = false;
		Ok(())
	}
}
