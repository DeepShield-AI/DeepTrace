use aya::programs;
use thiserror::Error;
/// Errors that can occur during eBPF operations
#[derive(Debug, Error)]
pub enum EbpfError {
	#[error("Failed to load eBPF program: {0}")]
	LoadError(#[from] aya::EbpfError),

	#[error("Failed to initialize eBPF logger: {0}")]
	LoggerError(String),

	#[error("Program '{0}' not found")]
	ProgramNotFound(String),

	#[error("Map '{0}' not found")]
	MapNotFound(String),

	#[error("Failed to attach program '{program}': {source}")]
	AttachError { program: String, source: aya::programs::ProgramError },

	#[error("Kernel version {actual} does not meet requirement {required} for {feature}")]
	KernelVersionMismatch { actual: String, required: String, feature: String },

	#[error("Hook point '{0}' not available on this kernel")]
	HookUnavailable(String),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Map operation failed: {0}")]
	MapError(String),

	#[error("Missing capability: {0}")]
	MissingCapability(String),

	#[error("Link id is wrong.")]
	WrongLinkId,

	#[error("Object error {0}.")]
	Object(#[from] object::Error),

	#[error("No {0} tracepoint category provided")]
	NoTracepointCategory(String),

	#[error("No attach function {0} provided")]
	NoAttachFunction(String),

	#[error("{0}")]
	Program(#[from] programs::ProgramError),
}

pub type Result<T> = std::result::Result<T, EbpfError>;
