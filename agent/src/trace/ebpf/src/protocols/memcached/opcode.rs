#![allow(clippy::upper_case_acronyms)]

use core::mem;

/// Memcache Opcodes
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
	Get = 0x00,
	Set = 0x01,
	Add = 0x02,
	Replace = 0x03,
	Delete = 0x04,
	Increment = 0x05,
	Decrement = 0x06,
	Quit = 0x07,
	Flush = 0x08,
	GetQ = 0x09,
	Noop = 0x0A,
	Version = 0x0B,
	GetK = 0x0C,
	GetKQ = 0x0D,
	Append = 0x0E,
	Prepare = 0x0F,
	Stat = 0x10,
	SetQ = 0x11,
	AddQ = 0x12,
	ReplaceQ = 0x13,
	DeleteQ = 0x14,
	IncrementQ = 0x15,
	DecrementQ = 0x16,
	QuitQ = 0x17,
	FlushQ = 0x18,
	AppendQ = 0x19,
	PrependQ = 0x1A,
	Verbosity = 0x1B,
	Touch = 0x1C,
	GAT = 0x1D,
	GATQ = 0x1E,

	SASLListMech = 0x20,
	SASLAuth = 0x21,
	SASLStep = 0x22,

	RGet = 0x30,
	RSet = 0x31,
	RSetQ = 0x32,
	RAppend = 0x33,
	RAppendQ = 0x34,
	RPrepend = 0x35,
	RPrependQ = 0x36,
	RDelete = 0x37,
	RDeleteQ = 0x38,
	RIncr = 0x39,
	RIncrQ = 0x3A,
	RDecr = 0x3B,
	RDecrQ = 0x3C,
	SetVBucket = 0x3D,
	GetVBucket = 0x3E,
	DelVBucket = 0x3F,
	TAPConnect = 0x40,
	TAPMutate = 0x41,
	TAPDelete = 0x42,
	TAPFlush = 0x43,
	TAPOpaque = 0x44,
	TAPVBucketSet = 0x45,
	TAPCheckpointStart = 0x46,
	TAPCheckpointEnd = 0x47,
}

impl TryFrom<u8> for OpCode {
	type Error = ();

	#[inline(always)]
	fn try_from(opcode: u8) -> Result<Self, Self::Error> {
		if (0x00..=0x1E).contains(&opcode) ||
			(0x20..=0x22).contains(&opcode) ||
			(0x30..=0x47).contains(&opcode)
		{
			return Ok(from_u8(opcode));
		}
		Err(())
	}
}

#[inline(always)]
fn from_u8(x: u8) -> OpCode {
	unsafe { mem::transmute(x) }
}
