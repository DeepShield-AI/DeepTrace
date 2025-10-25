use super::String;
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::collections::BTreeMap;

#[derive(Default, Clone)]
pub struct MetricTags(pub(crate) BTreeMap<String, String>);

impl FromIterator<(String, String)> for MetricTags {
	fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
		MetricTags(iter.into_iter().collect())
	}
}

impl MetricTags {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert<K: Into<String>, V: Into<String>>(&mut self, k: K, v: V) {
		self.0.insert(k.into(), v.into());
	}

	// pub fn get(&self, k: &str) -> Option<&Value> {
	// 	self.0.get(k)
	// }

	// pub fn is_empty(&self) -> bool {
	// 	self.0.is_empty()
	// }

	/// Replace all the values of a tag with a single value.
	pub fn replace<K: Into<String>, V: Into<String>>(&mut self, k: K, v: V) -> Option<String> {
		self.0.insert(k.into(), v.into())
	}
}

impl Serialize for MetricTags {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map_serializer = serializer.serialize_map(Some(self.0.len()))?;
		for (k, v) in &self.0 {
			map_serializer.serialize_entry(&k, &v)?;
		}
		map_serializer.end()
	}
}
