//! observ-event is a library for observability events.
//!
//! # Features
//!
//! - `metric`: Enable metric events.
//! - `message`: Enable message events.
//!

mod atomics;
mod macros;

#[cfg(feature = "metric")]
pub mod metric;

#[cfg(feature = "span")]
pub mod span;
