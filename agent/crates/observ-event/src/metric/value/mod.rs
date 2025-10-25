use serde::Serialize;

// mod counter;
mod gauge;
mod histogram;
mod summary;
// pub use counter::Counter;
pub use gauge::Gauge;
// use metrics::*;
/// Metric value
#[derive(Serialize)]
#[serde(untagged)]
pub enum MetricValue {
	// / A cumulative numerical value that can only increase or be reset to zero.
	// Counter(counter::Counter),
	/// A single numerical value that can arbitrarily go up and down.
	Gauge(Gauge),
	// /// String value
	// String(String),
}
