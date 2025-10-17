use thiserror::Error;

#[derive(Debug, Error)]
pub enum TraceError {
	#[error("ebpf error: {0}")]
	Ebpf(#[from] aya::EbpfError),
	#[error("Failed to open perf buffer: {0}")]
	PerfBuffer(#[from] aya::maps::perf::PerfBufferError),
	#[error("Failed to get current kernel version")]
	KernelVersionError,
	#[error("{0}")]
	EbpfManager(#[from] ebpf_manager::EbpfError),
	#[error("{0}")]
	Btf(#[from] aya::BtfError),
	#[error("{0}")]
	Map(#[from] aya::maps::MapError),
	#[error("{0}")]
	SpanConstructor(#[from] crate::span::SpanError),
}
