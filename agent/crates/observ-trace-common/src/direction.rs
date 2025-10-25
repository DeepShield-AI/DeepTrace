#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Direction {
	Ingress,
	Egress,
	Unknown,
}

#[cfg(feature = "user")]
impl std::fmt::Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Direction::Ingress => write!(f, "ingress"),
			Direction::Egress => write!(f, "egress"),
			Direction::Unknown => write!(f, "unknown"),
		}
	}
}
