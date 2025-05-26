use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to load eBPF program: {0}")]
	BTF(#[from] aya::BtfError),
	#[error("Failed to load eBPF program: {0}")]
	Program(#[from] aya::programs::ProgramError),
	#[error("Failed to load eBPF program: {0}")]
	Load(#[from] aya::EbpfError),
}
