use crate::Cow;
use serde::{Serialize, Serializer};

impl Serialize for Cow<'_, str> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self)
	}
}
