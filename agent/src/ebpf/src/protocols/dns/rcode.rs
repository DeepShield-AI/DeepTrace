/// Possible RCODE values for a DNS packet   
/// [RFC 1035](https://tools.ietf.org/html/rfc1035) Response code - this 4 bit field is set as part of responses.  
/// The values have the following interpretation
#[repr(u8)]
pub enum RCode {
	/// No error condition
	NoError = 0,
	/// Format error - The name server was unable to interpret the query.
	FormatError = 1,
	/// Server failure - The name server was unable to process this query due to a problem with the name server.
	ServerFailure = 2,
	/// Name Error - Meaningful only for responses from an authoritative name server,  
	/// this code signifies that the domain name referenced in the query does not exist.
	NameError = 3,
	/// Not Implemented - The name server does not support the requested kind of query.
	NotImplemented = 4,
	/// Refused - The name server refuses to perform the specified operation for policy reasons.  
	/// For example, a name server may not wish to provide the information to the particular requester,   
	/// or a name server may not wish to perform a particular operation (e.g., zone transfer) for particular data.
	Refused = 5,
	/// Some name that ought not to exist, does exist.
	/// [RFC 2136](https://datatracker.ietf.org/doc/html/rfc2136)
	YXDOMAIN = 6,
	/// Some RRset that ought not to exist, does exist.
	/// [RFC 2136](https://datatracker.ietf.org/doc/html/rfc2136)
	YXRRSET = 7,
	/// Some RRset that ought to exist, does not exist.
	/// [RFC 2136](https://datatracker.ietf.org/doc/html/rfc2136)
	NXRRSET = 8,
	/// The server is not authoritative for the zone named in the Zone Section.
	/// [RFC 2136](https://datatracker.ietf.org/doc/html/rfc2136)
	NOTAUTH = 9,
	/// A name used in the Prerequisite or Update Section is not within the zone denoted by the Zone Section.
	/// [RFC 2136](https://datatracker.ietf.org/doc/html/rfc2136)
	NOTZONE = 10,
	/// EDNS Version not supported by the responder
	/// [RFC 6891](https://datatracker.ietf.org/doc/html/rfc6891)
	BADVERS = 16,

	/// Reserved for future use.
	Reserved,
}
