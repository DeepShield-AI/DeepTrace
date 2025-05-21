use super::TraceError;
use aya::{Ebpf, EbpfLoader, VerifierLogLevel, include_bytes_aligned};

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

pub(super) fn load_trace() -> Result<Ebpf, TraceError> {
	let ebpf = EbpfLoader::new()
		// .verifier_log_level(log_level())
		.load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/trace")))?;
	Ok(ebpf)
}
