/// An object which can be converted into a `f64` representation.
///
/// This trait provides a mechanism for existing types, which have a natural representation
/// as a 64-bit floating-point number, to be transparently passed in when recording a histogram.
pub trait IntoF64 {
	/// Converts this object to its `f64` representation.
	fn into_f64(self) -> f64;
}

impl IntoF64 for u64 {
	fn into_f64(self) -> f64 {
		self as f64
	}
}

impl IntoF64 for &u64 {
	fn into_f64(self) -> f64 {
		*self as f64
	}
}

impl IntoF64 for f64 {
	fn into_f64(self) -> f64 {
		self
	}
}

impl IntoF64 for core::time::Duration {
	fn into_f64(self) -> f64 {
		self.as_secs_f64()
	}
}

/// Helper method to allow monomorphization of values passed to the `histogram!` macro.
#[doc(hidden)]
pub fn __into_f64<V: IntoF64>(value: V) -> f64 {
	value.into_f64()
}

macro_rules! into_f64 {
    ($($ty:ty),*) => {
        $(
            impl IntoF64 for $ty {
                fn into_f64(self) -> f64 {
                    f64::from(self)
                }
            }
        )*
    };
}

into_f64!(i8, u8, i16, u16, i32, u32, f32);
