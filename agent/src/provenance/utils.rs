use super::ProvenanceError;
use aya::{Btf, Ebpf, EbpfLoader, VerifierLogLevel, include_bytes_aligned, programs::Lsm};

fn log_level() -> VerifierLogLevel {
	match std::env::var("RUST_LOG") {
		Ok(s) => match s.as_str() {
			"debug" => VerifierLogLevel::DEBUG,
			"verbose" => VerifierLogLevel::VERBOSE,
			"disable" => VerifierLogLevel::DISABLE,
			_ => VerifierLogLevel::STATS,
		},
		_ => VerifierLogLevel::STATS,
	}
}

pub(super) fn load() -> Result<Ebpf, ProvenanceError> {
	let ebpf = EbpfLoader::new()
		.verifier_log_level(log_level())
		.load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/provenance")))?;
	Ok(ebpf)
}

pub(super) fn attach(ebpf: &mut Ebpf) -> Result<(), ProvenanceError> {
	let btf = Btf::from_sys_fs()?;
	let program: &mut Lsm = ebpf.program_mut("task_alloc").unwrap().try_into()?;
	program.load("task_alloc", &btf)?;
	program.attach()?;

	let program: &mut Lsm = ebpf.program_mut("task_free").unwrap().try_into()?;
	program.load("task_free", &btf)?;
	program.attach()?;
	Ok(())
}
