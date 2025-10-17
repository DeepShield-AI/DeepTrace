use super::{Infer, utils::check_protocol};
use constants::CASSANDRA_MIN_SIZE;
use flag::Flags;
use opcode::OpCode;

mod constants;
mod flag;
mod header;
mod opcode;
mod parse;

pub(crate) use header::Cassandra;
