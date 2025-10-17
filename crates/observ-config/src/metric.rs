use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct MetricConfig {
	/// sample interval in seconds
	#[serde(deserialize_with = "deserialize_seconds")]
	pub interval: Duration,
}

fn deserialize_seconds<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let secs = String::deserialize(deserializer)?
		.parse::<u64>()
		.map_err(serde::de::Error::custom)?;
	Ok(Duration::from_secs(secs))
}
