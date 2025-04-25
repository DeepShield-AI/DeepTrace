use bitflags::bitflags;

bitflags! {
	/// Possible Packet Flags
	/// ```markdown
	/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
	/// |QR|   Opcode  |AA|TC|RD|RA| Z|AD|CD|   RCODE   |
	/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
	/// ```
	pub struct PacketFlag: u16 {
		/// QR
		/// - A one bit field that specifies whether this message is a query (0), or a response (1).
		const Response = 1 << 15;
		/// AA
		/// Authoritative Answer - this bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question section.
		/// Note that the contents of the answer section may have multiple owner names because of aliases.  The AA bit corresponds to the name which matches the query name, or the first owner name in the answer section.
		const AuthoritativeAnswer = 1 << 10;
		/// TC
		/// TrunCation - specifies that this message was truncated due to length greater than that permitted on the transmission channel.
		const Truncated = 1 << 9;
		/// RD
		/// Recursion Desired may be set in a query and is copied into the response.  If RD is set, it directs the name server to pursue the query recursively.
		/// Recursive query support is optional.
		const RecursionDesired = 1 << 8;
		/// RA
		/// Recursion Available is set or cleared in a response, and denotes whether recursive query support is available in the name server.
		const RecursionAvailable = 1 << 7;
		const AuthenticData = 1 << 5;
		const CheckingDisabled = 1 << 4;
	}
}
pub(super) mod masks {
	pub const OPCODE_MASK: u16 = 0b0111_1000_0000_0000;
	pub const RESERVED_MASK: u16 = 0b0000_0000_0100_0000;
	pub const RESPONSE_CODE_MASK: u16 = 0b0000_0000_0000_1111;
}
