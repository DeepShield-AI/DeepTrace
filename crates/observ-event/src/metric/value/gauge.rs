use crate::{atomics::AtomicU64, metric::IntoF64};
use serde::Serialize;
use std::sync::{Arc, atomic::Ordering};

/// A gauge.
#[derive(Default)]
#[must_use = "gauges do nothing unless you use them"]
pub struct Gauge {
	inner: Option<Arc<dyn GaugeFn + Send + Sync>>,
}

impl Gauge {
	/// Creates a new `Gauge`.
	pub fn new() -> Self {
		Self { inner: Some(Arc::new(AtomicU64::new(0))) }
	}

	/// Creates a `Gauge` based on a shared handler.
	pub fn from_arc<F: GaugeFn + Send + Sync + 'static>(a: Arc<F>) -> Self {
		Self { inner: Some(a) }
	}

	/// Increments the gauge.
	pub fn increment<T: IntoF64>(&self, value: T) {
		if let Some(g) = &self.inner {
			g.increment(value.into_f64())
		}
	}

	/// Decrements the gauge.
	pub fn decrement<T: IntoF64>(&self, value: T) {
		if let Some(g) = &self.inner {
			g.decrement(value.into_f64())
		}
	}

	/// Sets the gauge.
	pub fn set<T: IntoF64>(&self, value: T) {
		if let Some(g) = &self.inner {
			g.set(value.into_f64())
		}
	}
}

/// A gauge handler.
pub trait GaugeFn {
	/// Increments the gauge by the given amount.
	fn increment(&self, value: f64);

	/// Decrements the gauge by the given amount.
	fn decrement(&self, value: f64);

	/// Sets the gauge to the given amount.
	fn set(&self, value: f64);

	/// Returns the current value of the gauge.
	fn value(&self) -> f64;
}

impl GaugeFn for AtomicU64 {
	fn increment(&self, value: f64) {
		loop {
			let result = self.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |curr| {
				let input = f64::from_bits(curr);
				let output = input + value;
				Some(output.to_bits())
			});

			if result.is_ok() {
				break;
			}
		}
	}

	fn decrement(&self, value: f64) {
		loop {
			let result = self.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |curr| {
				let input = f64::from_bits(curr);
				let output = input - value;
				Some(output.to_bits())
			});

			if result.is_ok() {
				break;
			}
		}
	}

	fn set(&self, value: f64) {
		let _ = self.swap(value.to_bits(), Ordering::AcqRel);
	}

	fn value(&self) -> f64 {
		f64::from_bits(self.load(Ordering::Acquire))
	}
}

impl Serialize for Gauge {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self.inner {
			Some(inner) => {
				let value = inner.as_ref().value();

				serializer.serialize_f64(value)
			},
			None => serializer.serialize_none(),
		}
	}
}
