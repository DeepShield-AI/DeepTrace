use super::{utils::check_protocol, Infer};
use flag::Flags;
use opcode::OpCode;

mod flag;
mod header;
mod opcode;
mod parse;

pub(crate) use header::Cassandra;
