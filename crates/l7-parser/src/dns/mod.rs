use super::{Infer, utils::check_protocol};
use constants::*;
use flag::PacketFlag;
use opcode::OpCode;
use rcode::RCode;

mod constants;
mod flag;
mod header;
mod opcode;
mod parse;
mod rcode;

pub(crate) use header::DNS;
