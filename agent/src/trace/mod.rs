pub mod attach;
mod error;
pub mod loader;
mod module;
mod span;
mod utils;

pub use error::Error as TraceError;
pub use module::TraceModule;
pub use span::{SpanConstructor, SpanError};
