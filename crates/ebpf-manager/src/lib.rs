//! # ebpf-loader
//!
//! A lightweight and practical eBPF program loader for observability applications.
//!
//! This crate provides utilities for:
//! - Loading eBPF programs with proper configuration
//! - Checking kernel version requirements for different hook types
//! - Verifying tracepoint and kprobe availability
//! - Managing eBPF logger initialization
//! - Helper functions for common operations
//!
//!
pub use error::EbpfError;
use error::Result;

mod elf;
mod error;
mod link;
pub mod log;
pub mod program;
pub mod utils;
mod version;
