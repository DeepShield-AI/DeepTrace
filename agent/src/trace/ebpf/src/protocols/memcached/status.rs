/// Status Field
///
/// Used in Response Packets if an error occurred
#[repr(u16)]
pub enum Status {
	NoError = 0,
	KeyNotFound = 1,
	KeyExists = 2,
	ValueTooLarge = 3,
	InvalidArguments = 4,
	ItemNotStored = 5,
	IncrDecrNonNumeric = 6,
	VBucketNotHere = 7,
	AuthError = 8,
	AuthContinue = 9,
	UnknownCommand = 0x81,
	OutOfMemory = 0x82,
	NotSupported = 0x83,
	InternalError = 0x84,
	Busy = 0x85,
	TemporaryFailure = 0x86,
}
