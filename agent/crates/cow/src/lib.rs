#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use core::{
	borrow::Borrow,
	cmp::Ordering,
	fmt,
	hash::{Hash, Hasher},
	marker::PhantomData,
	mem::ManuallyDrop,
	ops::Deref,
	ptr::{NonNull, slice_from_raw_parts},
};

#[cfg(feature = "serde")]
mod serde;

pub struct Cow<'a, T: Cowable + ?Sized + 'a> {
	/// Pointer to the data.
	ptr: NonNull<T::Pointer>,
	metadata: Metadata,
	_marker: PhantomData<&'a T>,
}

impl<T> Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	const fn from_parts(ptr: NonNull<T::Pointer>, metadata: Metadata) -> Self {
		Self { ptr, metadata, _marker: PhantomData }
	}

	/// Creates a pointer to an owned value, consuming it.
	pub fn from_owned(owned: T::Owned) -> Self {
		let (ptr, metadata) = T::owned_into_parts(owned);

		// This check is partially to guard against the semantics of `Vec<T>` changing in the
		// future, and partially to ensure that we don't somehow implement `Cowable` for a type
		// where its owned version is backed by a vector of ZSTs, where the capacity could
		// _legitimately_ be `usize::MAX`.
		if metadata.capacity() == usize::MAX {
			panic!("Invalid capacity of `usize::MAX` for owned value.");
		}

		Self::from_parts(ptr, metadata)
	}

	/// Creates a pointer to a shared value.
	pub fn from_shared(arc: Arc<T>) -> Self {
		let (ptr, metadata) = T::shared_into_parts(arc);
		Self::from_parts(ptr, metadata)
	}

	/// Extracts the owned data.
	///
	/// Clones the data if it is not already owned.
	pub fn into_owned(self) -> <T as ToOwned>::Owned {
		// We need to ensure that our own `Drop` impl does _not_ run because we're simply
		// transferring ownership of the value back to the caller. For borrowed values, this is
		// naturally a no-op because there's nothing to drop, but for owned values, like `String` or
		// `Arc<T>`, we wouldn't want to double drop.
		let cow = ManuallyDrop::new(self);

		T::owned_from_parts(cow.ptr, &cow.metadata)
	}
}

impl<'a, T> Cow<'a, T>
where
	T: Cowable + ?Sized,
{
	/// Creates a pointer to a borrowed value.
	pub fn from_borrowed(borrowed: &'a T) -> Self {
		let (ptr, metadata) = T::borrowed_into_parts(borrowed);

		Self::from_parts(ptr, metadata)
	}
}

impl<'a, T> Cow<'a, [T]>
where
	T: Clone,
{
	pub const fn const_slice(val: &'a [T]) -> Cow<'a, [T]> {
		// SAFETY: We can never create a null pointer by casting a reference to a pointer.
		let ptr = unsafe { NonNull::new_unchecked(val.as_ptr() as *mut _) };
		let metadata = Metadata::borrowed(val.len());

		Self { ptr, metadata, _marker: PhantomData }
	}
}

impl<'a> Cow<'a, str> {
	pub const fn const_str(val: &'a str) -> Self {
		// SAFETY: We can never create a null pointer by casting a reference to a pointer.
		let ptr = unsafe { NonNull::new_unchecked(val.as_ptr() as *mut _) };
		let metadata = Metadata::borrowed(val.len());

		Self { ptr, metadata, _marker: PhantomData }
	}
}

impl<T> Deref for Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		let borrowed_ptr = T::borrowed_from_parts(self.ptr, self.metadata.len());

		// SAFETY: We only ever hold a pointer to a borrowed value of at least the lifetime of
		// `Self`, or an owned value which we have ownership of (albeit indirectly when using
		// `Arc<T>`), so our pointer is always valid and live for dereferencing.
		unsafe { borrowed_ptr.as_ref().unwrap() }
	}
}

impl<T> Clone for Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	fn clone(&self) -> Self {
		let (ptr, metadata) = T::clone_from_parts(self.ptr, &self.metadata);
		Self { ptr, metadata, _marker: PhantomData }
	}
}

impl<T> Drop for Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	fn drop(&mut self) {
		T::drop_from_parts(self.ptr, &self.metadata);
	}
}

impl<T> Hash for Cow<'_, T>
where
	T: Hash + Cowable + ?Sized,
{
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.deref().hash(state)
	}
}

impl<'a, T> Default for Cow<'a, T>
where
	T: Cowable + ?Sized,
	&'a T: Default,
{
	#[inline]
	fn default() -> Self {
		Cow::from_borrowed(Default::default())
	}
}

impl<T> Eq for Cow<'_, T> where T: Eq + Cowable + ?Sized {}

impl<A, B> PartialOrd<Cow<'_, B>> for Cow<'_, A>
where
	A: Cowable + ?Sized + PartialOrd<B>,
	B: Cowable + ?Sized,
{
	#[inline]
	fn partial_cmp(&self, other: &Cow<'_, B>) -> Option<Ordering> {
		PartialOrd::partial_cmp(self.deref(), other.deref())
	}
}

impl<T> Ord for Cow<'_, T>
where
	T: Ord + Cowable + ?Sized,
{
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(self.deref(), other.deref())
	}
}

impl<'a, T> From<&'a T> for Cow<'a, T>
where
	T: Cowable + ?Sized,
{
	#[inline]
	fn from(val: &'a T) -> Self {
		Cow::from_borrowed(val)
	}
}

impl From<usize> for Cow<'_, str> {
	#[inline]
	fn from(val: usize) -> Self {
		Cow::from_owned(val.to_string())
	}
}

impl<T> From<Arc<T>> for Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	#[inline]
	fn from(val: Arc<T>) -> Self {
		Cow::from_shared(val)
	}
}

impl From<String> for Cow<'_, str> {
	#[inline]
	fn from(s: String) -> Self {
		Cow::from_owned(s)
	}
}

impl<T> From<Vec<T>> for Cow<'_, [T]>
where
	T: Clone,
{
	#[inline]
	fn from(v: Vec<T>) -> Self {
		Cow::from_owned(v)
	}
}

impl<T> AsRef<T> for Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	#[inline]
	fn as_ref(&self) -> &T {
		self.borrow()
	}
}

impl<T> Borrow<T> for Cow<'_, T>
where
	T: Cowable + ?Sized,
{
	#[inline]
	fn borrow(&self) -> &T {
		self.deref()
	}
}

impl<A, B> PartialEq<Cow<'_, B>> for Cow<'_, A>
where
	A: Cowable + ?Sized,
	B: Cowable + ?Sized,
	A: PartialEq<B>,
{
	fn eq(&self, other: &Cow<B>) -> bool {
		self.deref() == other.deref()
	}
}

impl<T> fmt::Debug for Cow<'_, T>
where
	T: Cowable + fmt::Debug + ?Sized,
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.deref().fmt(f)
	}
}

impl<T> fmt::Display for Cow<'_, T>
where
	T: Cowable + fmt::Display + ?Sized,
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.deref().fmt(f)
	}
}

// SAFETY: `NonNull<T>` is not `Send` or `Sync` by default, but we're asserting that `Cow` is so
// long as the underlying `T` is.
unsafe impl<T: Cowable + Sync + ?Sized> Sync for Cow<'_, T> {}
unsafe impl<T: Cowable + Send + ?Sized> Send for Cow<'_, T> {}

enum Kind {
	Owned,
	Borrowed,
	Shared,
}

#[derive(Copy, Clone)]
pub struct Metadata(usize, usize);

impl Metadata {
	#[inline]
	const fn len(&self) -> usize {
		self.0
	}

	#[inline]
	const fn capacity(&self) -> usize {
		self.1
	}

	#[inline]
	const fn kind(&self) -> Kind {
		match (self.0, self.1) {
			(_, usize::MAX) => Kind::Shared,
			(_, 0) => Kind::Borrowed,
			_ => Kind::Owned,
		}
	}

	#[inline]
	const fn borrowed(len: usize) -> Metadata {
		Metadata(len, 0)
	}

	#[inline]
	const fn owned(len: usize, capacity: usize) -> Metadata {
		Metadata(len, capacity)
	}

	#[inline]
	const fn shared(len: usize) -> Metadata {
		Metadata(len, usize::MAX)
	}
}
pub trait Cowable: ToOwned {
	type Pointer;
	fn borrowed_into_parts(&self) -> (NonNull<Self::Pointer>, Metadata);

	fn borrowed_from_parts(ptr: NonNull<Self::Pointer>, len: usize) -> *const Self;

	/// Convert `T::Owned` to `NonNull<T>` and capacity.
	/// Return `None` for `0` capacity.
	fn owned_into_parts(owned: Self::Owned) -> (NonNull<Self::Pointer>, Metadata);

	/// Rebuild `T::Owned` from `NonNull<T>` and `capacity`. This can be done by the likes
	/// of [`Vec::from_raw_parts`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts).
	fn owned_from_parts(ptr: NonNull<Self::Pointer>, metadata: &Metadata) -> Self::Owned;

	fn shared_into_parts(arc: Arc<Self>) -> (NonNull<Self::Pointer>, Metadata);

	fn clone_from_parts(
		ptr: NonNull<Self::Pointer>,
		metadata: &Metadata,
	) -> (NonNull<Self::Pointer>, Metadata);

	fn drop_from_parts(ptr: NonNull<Self::Pointer>, metadata: &Metadata);
}

impl Cowable for str {
	type Pointer = u8;

	#[inline]
	fn borrowed_into_parts(&self) -> (NonNull<Self::Pointer>, Metadata) {
		// SAFETY: We know that it's safe to take and hold a pointer to a reference to `Self` since
		// `Cow` can only live as long as the input reference does, and an invalid pointer cannot
		// be taken from a live reference.
		let ptr = unsafe { NonNull::new_unchecked(self.as_ptr() as *mut _) };
		(ptr, Metadata::borrowed(self.len()))
	}

	#[inline]
	fn borrowed_from_parts(ptr: NonNull<Self::Pointer>, length: usize) -> *const Self {
		slice_from_raw_parts(ptr.as_ptr(), length) as *const _
	}

	#[inline]
	fn owned_into_parts(owned: Self::Owned) -> (NonNull<Self::Pointer>, Metadata) {
		// SAFETY: We know that it's safe to take and hold a pointer to a reference to `owned` since
		// we own the allocation by virtue of consuming it here without dropping it.
		let mut owned = ManuallyDrop::new(owned.into_bytes());
		let ptr = unsafe { NonNull::new_unchecked(owned.as_mut_ptr()) };
		let metadata = Metadata::owned(owned.len(), owned.capacity());
		(ptr, metadata)
	}

	#[inline]
	fn shared_into_parts(arc: Arc<Self>) -> (NonNull<Self::Pointer>, Metadata) {
		let metadata = Metadata::shared(arc.len());
		// SAFETY: We know that the pointer given back by `Arc::into_raw` is valid.
		let ptr = unsafe { NonNull::new_unchecked(Arc::into_raw(arc) as *mut _) };
		(ptr, metadata)
	}

	#[inline]
	fn owned_from_parts(
		ptr: NonNull<Self::Pointer>,
		metadata: &Metadata,
	) -> <Self as ToOwned>::Owned {
		match metadata.kind() {
			Kind::Borrowed => {
				// SAFETY: We know that it's safe to take and hold a pointer to a reference to
				// `Self` since `Cow` can only live as long as the input reference does, and an
				// invalid pointer cannot be taken from a live reference.
				let s = unsafe { &*Self::borrowed_from_parts(ptr, metadata.len()) };
				s.to_owned()
			},

			// SAFETY: We know that the pointer is valid because it could have only been constructed
			// from a valid `String` handed to `Cow::from_owned`, which we assumed ownership of.
			Kind::Owned => unsafe {
				String::from_raw_parts(ptr.as_ptr(), metadata.len(), metadata.capacity())
			},
			Kind::Shared => {
				// SAFETY: We know that the pointer is valid because it could have only been
				// constructed from a valid `Arc<str>` handed to `Cow::from_shared`, which we
				// assumed ownership of, also ensuring that the strong count is at least one.
				let s = unsafe { Arc::from_raw(Self::borrowed_from_parts(ptr, metadata.len())) };
				s.to_string()
			},
		}
	}

	#[inline]
	fn clone_from_parts(
		ptr: NonNull<Self::Pointer>,
		metadata: &Metadata,
	) -> (NonNull<Self::Pointer>, Metadata) {
		match metadata.kind() {
			Kind::Borrowed => (ptr, *metadata),
			Kind::Owned => {
				// SAFETY: We know that the pointer is valid because it could have only been constructed
				// from a valid `String` handed to `Cow::from_owned`, which we assumed ownership of.
				let s = unsafe { &*Self::borrowed_from_parts(ptr, metadata.len()) };

				Self::owned_into_parts(s.to_string())
			},
			Kind::Shared => clone_shared::<Self>(ptr, metadata),
		}
	}

	#[inline]
	fn drop_from_parts(ptr: NonNull<Self::Pointer>, metadata: &Metadata) {
		match metadata.kind() {
			Kind::Borrowed => {},

			// SAFETY: We know that the pointer is valid because it could have only been constructed
			// from a valid `String` handed to `Cow::from_owned`, which we assumed ownership of.
			Kind::Owned => unsafe {
				drop(Vec::from_raw_parts(ptr.as_ptr(), metadata.len(), metadata.capacity()));
			},

			// SAFETY: We know that the pointer is valid because it could have only been constructed
			// from a valid `Arc<str>` handed to `Cow::from_shared`, which we assumed ownership of,
			// also ensuring that the strong count is at least one.
			Kind::Shared => unsafe {
				drop(Arc::from_raw(Self::borrowed_from_parts(ptr, metadata.len())));
			},
		}
	}
}

impl<T> Cowable for [T]
where
	T: Clone,
{
	type Pointer = T;

	#[inline]
	fn borrowed_into_parts(&self) -> (NonNull<Self::Pointer>, Metadata) {
		// SAFETY: We know that it's safe to take and hold a pointer to a reference to `Self` since
		// `Cow` can only live as long as the input reference does, and an invalid pointer cannot
		// be taken from a live reference.
		let ptr = unsafe { NonNull::new_unchecked(self.as_ptr() as *mut _) };
		let metadata = Metadata::borrowed(self.len());
		(ptr, metadata)
	}

	#[inline]
	fn owned_into_parts(owned: <Self as ToOwned>::Owned) -> (NonNull<Self::Pointer>, Metadata) {
		let mut owned = ManuallyDrop::new(owned);

		// SAFETY: We know that it's safe to take and hold a pointer to a reference to `owned` since
		// we own the allocation by virtue of consuming it here without dropping it.
		let ptr = unsafe { NonNull::new_unchecked(owned.as_mut_ptr()) };
		let metadata = Metadata::owned(owned.len(), owned.capacity());
		(ptr, metadata)
	}

	#[inline]
	fn shared_into_parts(arc: Arc<Self>) -> (NonNull<Self::Pointer>, Metadata) {
		let metadata = Metadata::shared(arc.len());
		// SAFETY: We know that the pointer given back by `Arc::into_raw` is valid.
		let ptr = unsafe { NonNull::new_unchecked(Arc::into_raw(arc) as *mut _) };
		(ptr, metadata)
	}

	#[inline]
	fn borrowed_from_parts(ptr: NonNull<Self::Pointer>, length: usize) -> *const Self {
		slice_from_raw_parts(ptr.as_ptr(), length) as *const _
	}

	#[inline]
	fn owned_from_parts(
		ptr: NonNull<Self::Pointer>,
		metadata: &Metadata,
	) -> <Self as ToOwned>::Owned {
		match metadata.kind() {
			Kind::Borrowed => {
				// SAFETY: We know that it's safe to take and hold a pointer to a reference to
				// `Self` since `Cow` can only live as long as the input reference does, and an
				// invalid pointer cannot be taken from a live reference.
				let data = unsafe { &*Self::borrowed_from_parts(ptr, metadata.len()) };
				data.to_vec()
			},

			// SAFETY: We know that the pointer is valid because it could have only been
			// constructed from a valid `Vec<T>` handed to `Cow::from_owned`, which we
			// assumed ownership of.
			Kind::Owned => unsafe {
				Vec::from_raw_parts(ptr.as_ptr(), metadata.len(), metadata.capacity())
			},

			Kind::Shared => {
				// SAFETY: We know that the pointer is valid because it could have only been
				// constructed from a valid `Arc<[T]>` handed to `Cow::from_shared`, which we
				// assumed ownership of, also ensuring that the strong count is at least one.
				let arc = unsafe { Arc::from_raw(Self::borrowed_from_parts(ptr, metadata.len())) };
				arc.to_vec()
			},
		}
	}

	#[inline]
	fn clone_from_parts(
		ptr: NonNull<Self::Pointer>,
		metadata: &Metadata,
	) -> (NonNull<Self::Pointer>, Metadata) {
		match metadata.kind() {
			Kind::Borrowed => (ptr, *metadata),
			Kind::Owned => {
				let vec_ptr = Self::borrowed_from_parts(ptr, metadata.len());

				// SAFETY: We know that the pointer is valid because it could have only been
				// constructed from a valid `Vec<T>` handed to `Cow::from_owned`, which we assumed
				// ownership of.
				let new_vec = unsafe { vec_ptr.as_ref().unwrap().to_vec() };

				Self::owned_into_parts(new_vec)
			},
			Kind::Shared => clone_shared::<Self>(ptr, metadata),
		}
	}

	#[inline]
	fn drop_from_parts(ptr: NonNull<Self::Pointer>, metadata: &Metadata) {
		match metadata.kind() {
			Kind::Borrowed => {},

			// SAFETY: We know that the pointer is valid because it could have only been constructed
			// from a valid `Vec<T>` handed to `Cow::from_owned`, which we assumed ownership of.
			Kind::Owned => unsafe {
				drop(Vec::from_raw_parts(ptr.as_ptr(), metadata.len(), metadata.capacity()));
			},

			// SAFETY: We know that the pointer is valid because it could have only been constructed
			// from a valid `Arc<[T]>` handed to `Cow::from_shared`, which we assumed ownership of,
			// also ensuring that the strong count is at least one.
			Kind::Shared => unsafe {
				drop(Arc::from_raw(Self::borrowed_from_parts(ptr, metadata.len())));
			},
		}
	}
}

fn clone_shared<T: Cowable + ?Sized>(
	ptr: NonNull<T::Pointer>,
	metadata: &Metadata,
) -> (NonNull<T::Pointer>, Metadata) {
	let arc_ptr = T::borrowed_from_parts(ptr, metadata.len());

	// SAFETY: We know that the pointer is valid because it could have only been
	// constructed from a valid `Arc<T>` handed to `Cow::from_shared`, which we assumed
	// ownership of, also ensuring that the strong count is at least one.
	unsafe {
		Arc::increment_strong_count(arc_ptr);
	}

	(ptr, *metadata)
}
