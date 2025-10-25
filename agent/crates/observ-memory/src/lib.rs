//! Collect memory metrics.
pub use error::MemoryMetricError;

mod error;
pub mod meminfo;
pub mod vmstat;
