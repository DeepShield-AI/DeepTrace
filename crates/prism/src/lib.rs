#![recursion_limit = "256"] // for async-stream
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::broken_intra_doc_links))]

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
pub use agent::Agent;
pub use error::AgentError;
pub use observ_core::Module;

mod agent;
mod error;
