// This file exists to enable the library target.
#![cfg_attr(not(test), no_std)]
#![allow(dead_code, static_mut_refs, unused_imports)]
mod maps;
pub mod network;
pub mod process;
mod vmlinux;
