use core::mem;

/// Clients can send request messages that specify all but the [OpReply](OpCode::OpReply) OpCode. `OpReply` is reserved for use by the database.
/// Only the [OpQuery](OpCode::OpQuery) and [OpGetMore](OpCode::OpGetMore) messages result in a response from the database. There will be no response sent for any other message.
#[repr(i32)]
#[derive(Debug)]
pub(crate) enum OpCode {
	/// Reply to a client request. `response_to` is set.
	OpReply = 1,
	/// Update document.
	OpUpdate = 2001,
	/// Insert new document.
	OpInsert = 2002,
	/// Formerly used for `OP_GET_BY_OID`.
	Reserved = 2003,
	/// Query a collection.
	OpQuery = 2004,
	/// Get more data from a query.
	OpGetMore = 2005,
	/// Delete documents.
	OpDelete = 2006,
	/// Notify database that the client has finished with the cursor.
	OpKillCursors = 2007,
	/// Wraps other opcodes using compression
	OpCompressed = 2012,
	/// Send a message using the format introduced in MongoDB 3.6.
	OpMsg = 2013,
}

impl TryFrom<i32> for OpCode {
	type Error = ();

	#[inline(always)]
	fn try_from(opcode: i32) -> Result<Self, Self::Error> {
		if opcode == 1 || (2001..=2007).contains(&opcode) || (2012..=2013).contains(&opcode) {
			return Ok(from_i32(opcode));
		}
		Err(())
	}
}

#[inline(always)]
fn from_i32(x: i32) -> OpCode {
	unsafe { mem::transmute(x) }
}
