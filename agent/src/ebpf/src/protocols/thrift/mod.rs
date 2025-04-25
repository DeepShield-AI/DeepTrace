use super::Infer;
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use kind::Kind;
use mercury_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::Quintuple,
};

mod constants;
mod kind;
mod parse;
use crate::protocols::utils::check_protocol;
use parse::{thrift_binary_header, thrift_compact_header};

#[derive(Debug)]
pub(crate) struct Thrift {
	pub kind: Kind,
}

impl Thrift {
	fn message_type(&self) -> MessageType {
		match self.kind {
			Kind::Call | Kind::Oneway => MessageType::Request,
			Kind::Reply | Kind::Exception => MessageType::Response,
		}
	}
}

impl Infer for Thrift {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		let payload = info.buf.as_slice();
		if !check_protocol(info.key, L7Protocol::Thrift) {
			return Err(0);
		}
		match thrift_binary_header(payload, info.count) {
			Ok(thrift) => {
				let mut message = Message::new();
				message.protocol = L7Protocol::Thrift;
				message.type_ = thrift.message_type();
				Ok(message)
			},
			Err(_) => match thrift_compact_header(payload) {
				Ok(thrift) => {
					let mut message = Message::new();
					message.protocol = L7Protocol::Thrift;
					message.type_ = thrift.message_type();
					Ok(message)
				},
				Err(_) => Err(0),
			},
		}
	}
}
