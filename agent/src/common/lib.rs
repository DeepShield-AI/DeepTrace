// #![no_std]
#![cfg_attr(not(feature = "user"), no_std)]
pub mod protocol;
mod structs;

pub const MAX_PAYLOAD_SIZE: u32 = 1 << 14;
pub const TASK_CMD_LEN: usize = 16;
pub const MAX_PRE_PAYLOAD_SIZE: u32 = 1 << 3;
pub use protocol::*;
pub use structs::{Data, Quintuple, SyscallName, SyscallType};
