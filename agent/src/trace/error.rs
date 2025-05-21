use aya::{EbpfError, maps};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to load eBPF program: {0}")]
	Load(#[from] EbpfError),
	#[error("Failed to access eBPF map: {0}")]
	Map(#[from] maps::MapError),
}
