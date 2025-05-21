mod constants;
mod header;
mod opcode;
mod parse;
mod status;
#[cfg(test)]
mod tests;

use super::{utils::check_protocol, Infer};
use constants::*;
pub(crate) use header::Memcached;
use opcode::OpCode;
