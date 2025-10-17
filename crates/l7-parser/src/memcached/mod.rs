mod constants;
mod header;
mod opcode;
mod parse;
mod status;
#[cfg(test)]
mod tests;

use super::{Infer, utils::check_protocol};
use constants::*;
pub(crate) use header::Memcached;
use opcode::OpCode;
