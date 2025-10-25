mod tags;
mod value;
use chrono::{DateTime, Local};
use cow::Cow;
use serde::Serialize;
pub use tags::MetricTags;
pub use value::*;
mod common;
use bytes::BytesMut;
pub(in crate::metric) use common::IntoF64;

pub type String = Cow<'static, str>;

#[derive(Serialize)]
pub struct Metric {
	name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	namespace: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	tags: Option<MetricTags>,
	timestamp: DateTime<Local>,
	value: MetricValue,
}

impl Metric {
	#[inline]
	pub fn new<T: Into<String>>(name: T, value: MetricValue) -> Self {
		Self { name: name.into(), namespace: None, tags: None, timestamp: Local::now(), value }
	}

	/// Consumes this metric, returning it with an updated series based on the given `namespace`.
	#[inline]
	#[must_use]
	pub fn with_namespace<T: Into<String>>(mut self, namespace: Option<T>) -> Self {
		self.namespace = namespace.map(Into::into);
		self
	}

	/// Consumes this metric, returning it with an updated series based on the given `tags`.
	#[inline]
	#[must_use]
	pub fn with_tags(mut self, tags: Option<MetricTags>) -> Self {
		self.tags = tags;
		self
	}
}

impl Metric {
	#[inline]
	pub fn encode(&self, o: &mut BytesMut) -> Result<(), std::io::Error> {
		let value = serde_json::to_string(&self.value)
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

		let tag = self
			.tags
			.as_ref()
			.map(|t| serde_json::to_string(t).unwrap_or_else(|_| std::string::String::new()))
			.unwrap_or_else(|| std::string::String::new())
			.replace(',', ";");

		let namespace = self.namespace.as_ref().map(|s| s.as_ref()).unwrap_or("");
		let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

		let csv_line = format!("{},{},{},{},{}", self.name, namespace, value, tag, timestamp);

		o.extend_from_slice(csv_line.as_bytes());
		Ok(())
	}
}
