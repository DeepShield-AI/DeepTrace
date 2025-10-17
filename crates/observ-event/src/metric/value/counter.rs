use std::sync::Arc;

// use serde::Serialize;

// pub struct Counter {
// 	inner: Option<Arc<dyn CounterFn + Send + Sync>>,
// }

// impl Serialize for Counter {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match &self.inner {
//             Some(a) => serializer.serialize_u64(atomic_load(a)),
//             None => serializer.serialize_u64(0),
//         }
//     }
// }
/// A counter handler.
// pub trait CounterFn {
// 	/// Increments the counter by the given amount.
// 	fn increment(&self, value: u64);

// 	/// Sets the counter to at least the given amount.
// 	///
// 	/// This is intended to support use cases where multiple callers are attempting to synchronize
// 	/// this counter with an external counter that they have no control over.  As multiple callers
// 	/// may read that external counter, and attempt to set it here, there could be reordering issues
// 	/// where a caller attempts to set an older (smaller) value after the counter has been updated to
// 	/// the latest (larger) value.
// 	///
// 	/// This method must cope with those cases.  An example of doing so atomically can be found in
// 	/// `AtomicCounter`.
// 	fn absolute(&self, value: u64);
// }

// use crate::metric::;

// impl CounterFn for AtomicU64 {
// 	fn increment(&self, value: u64) {
// 		let _ = self.fetch_add(value, Ordering::Release);
// 	}

// 	fn absolute(&self, value: u64) {
// 		let _ = self.fetch_max(value, Ordering::AcqRel);
// 	}
// }
