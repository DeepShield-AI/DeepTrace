//! Core types and traits for observability data processing.
//!! This crate defines the core abstractions and data structures used throughout the
//! observability system, including modules, senders, and sendable data types.

pub use module::Module;
pub use sender::{Sendable, Sender};

mod module;
mod sender;
