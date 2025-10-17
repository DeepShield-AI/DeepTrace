//! Allocator for eBPF programs.
#![allow(static_mut_refs)]

use crate::{
	buffer::Buffer,
	constants::MAX_PAYLOAD_SIZE,
	error::{Result, code::*},
};
use aya_ebpf::{
	macros::map,
	maps::{PerCpuArray, PerCpuHashMap},
};
use core::mem::size_of;

const fn max(a: usize, b: usize) -> usize {
	if a < b {
		return b;
	}
	a
}

macro_rules! max {
    ($a:expr $(,)?) => ($a);
    ($a:expr, $($rest:expr),* $(,)?) => {{max($a, max!($($rest),*))}};
}

const MAX_ALLOCS: u32 = 1;

// Optimized [`HEAP_MAX_ALLOC_SIZE`]
// we need to double the actual maximum size we need to hack the verifier.
// It is sometimes impossible for the verifier to evaluate the correct bound
// check. This is the case for self modifying structure (appending/prepending
// operations). The verifier always need a constant value to bound a probe_read
// operation, however when appending the size we actually can write is variable
// and this causes a lot of troubles to the verifier. It is not always possible
// to fix an acceptable bound for size in probe_read, so a hack is to double
// the size of the map value used to allocate such a structure. In this way,
// the verifier always think there is enough room to write data. Special care
// to the bound checks must be taken because it may overrun the structures
// without triping up the verifier.
const HEAP_MAX_ALLOC_SIZE: usize = max!(
	(4 + 16) * size_of::<u8>() +
		(4 + 1) * size_of::<u16>() +
		(6 + 2) * size_of::<u32>() +
		size_of::<u64>() +
		size_of::<Buffer<MAX_PAYLOAD_SIZE>>()
) * 2;

const ZEROS: [u8; HEAP_MAX_ALLOC_SIZE] = [0; HEAP_MAX_ALLOC_SIZE];

// allocator is much faster with a PerCpuHashMap filled out with ZEROS
// elements rather than using a PerCpuArray + memset 0
#[map]
static mut HEAP: PerCpuHashMap<u32, [u8; HEAP_MAX_ALLOC_SIZE]> =
	PerCpuHashMap::with_max_entries(MAX_ALLOCS, 0);

#[map]
static mut ALLOCATOR: PerCpuArray<Allocator> = PerCpuArray::with_max_entries(1, 0);

pub struct Allocator {
	pub next: u32,
}

#[inline(always)]
pub fn init() -> Result<()> {
	Allocator::new()?;
	Ok(())
}

#[inline(always)]
pub fn alloc_zero<T>() -> Result<&'static mut T> {
	let alloc = Allocator::reuse()?;
	alloc.zero_alloc::<T>()
}

impl Allocator {
	// TODO: can we remove return value here?
	fn new() -> Result<&'static mut Self> {
		let a = Self::reuse()?;
		a.next = 0;
		Ok(a)
	}

	fn reuse() -> Result<&'static mut Self> {
		let ptr = unsafe { &ALLOCATOR }.get_ptr_mut(0).ok_or(FAILED_TO_GET_ALLOCATOR)?;
		let a = unsafe { &mut *ptr };
		Ok(a)
	}

	#[inline(always)]
	fn alloc_slice<T>(&mut self) -> Result<&'static mut [u8]> {
		let sizeof = size_of::<T>();

		if self.next == MAX_ALLOCS {
			return Err(ALLOC_NO_SPACE);
		}
		let k = self.next;
		unsafe { HEAP.insert(&k, &ZEROS, 0).map_err(|_| ALLOC_ZERO_CHUNK_FAILED)? };

		if let Some(alloc) = unsafe { HEAP.get_ptr_mut(&k).and_then(|a| a.as_mut()) } {
			if sizeof > alloc.len() {
				return Err(ALLOC_TOO_BIG);
			}

			self.next += 1;

			return Ok(alloc);
		}

		Err(ALLOC_NO_SPACE)
	}

	fn zero_alloc<T>(&mut self) -> Result<&'static mut T> {
		let alloc = self.alloc_slice::<T>()?;
		Ok(unsafe { core::mem::transmute(alloc.as_mut_ptr()) })
	}
}
