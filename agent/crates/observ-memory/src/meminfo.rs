//! Memory info
//!
//! Provides information about distribution and utilization of memory. This
//! varies by architecture and compile options.
#![allow(non_snake_case)]

use super::MemoryMetricError;
use observ_macros::ProcParser;
use observ_metric_common::procfs_root;
use observ_metric_utils::read_to_string;
use uom::si::{f64::Information, information::kilobyte};

#[derive(ProcParser)]
#[fmt = "kv"]
pub struct MemInfo {
	/// Total usable RAM (i.e., physical RAM minus a few reserved bits and the
	/// kernel binary code).
	#[arg(unit = kilobyte)]
	MemTotal: Information,
	/// The sum of [`LowFree`] + [`HighFree`]
	#[arg(unit = kilobyte)]
	MemFree: Information,
	/// (since Linux 3.14)
	/// An estimate of how much memory is available for starting new
	/// applications, without swapping.
	#[arg(unit = kilobyte)]
	MemAvailable: Information,
	/// Relatively temporary storage for raw disk blocks that shouldn't get
	/// tremendously large (20 MB or so)
	#[arg(unit = kilobyte)]
	Buffers: Information,
	/// In-memory cache for files read from the disk (the page cache). Doesn't
	/// include [`SwapCached`](Self::SwapCached).
	#[arg(unit = kilobyte)]
	Cached: Information,
	/// Memory that once was swapped out, is swapped back in but still also is
	/// in the swap file. (If memory pressure is high, these pages don't need
	/// to be swapped out again because they are already in the swap file. This
	/// saves I/O.)
	#[arg(unit = kilobyte)]
	SwapCached: Information,
	/// Memory that has been used more recently and usually not reclaimed
	/// unless absolutely necessary.
	#[arg(unit = kilobyte)]
	Active: Information,
	/// Memory which has been less recently used. It is more eligible to be
	/// reclaimed for other purposes.
	#[arg(unit = kilobyte)]
	Inactive: Information,
	/// (since Linux 2.6.28)
	#[arg(key = "Active(anon)", unit = kilobyte)]
	Active_anon: Information,
	/// (since Linux 2.6.28)
	#[arg(key = "Inactive(anon)", unit = kilobyte)]
	Inactive_anon: Information,
	/// (since Linux 2.6.28)
	#[arg(key = "Active(file)", unit = kilobyte)]
	Active_file: Information,
	/// (since Linux 2.6.28)
	#[arg(key = "Inactive(file)", unit = kilobyte)]
	Inactive_file: Information,
	/// Total amount of swap space available.
	#[arg(unit = kilobyte)]
	SwapTotal: Information,
	/// Amount of swap space that is currently unused.
	#[arg(unit = kilobyte)]
	SwapFree: Information,
	/// Memory which is waiting to get written back to the disk.
	#[arg(unit = kilobyte)]
	Dirty: Information,
	/// Writeback
	#[arg(unit = kilobyte)]
	Writeback: Information,
	/// (since Linux 2.6.18)
	/// Non-file backed pages mapped into user-space page tables.
	#[arg(unit = kilobyte)]
	AnonPages: Information,
	/// Files which have been mapped into memory (with mmap(2)), such as
	/// libraries.
	#[arg(unit = kilobyte)]
	Mapped: Information,
	/// (since Linux 2.6.32)
	/// Amount of memory consumed in tmpfs(5) filesystems, System V, and POSIX
	/// shared memory, as well as shared anonymous mappings
	/// (MAP_SHARED|MAP_ANONYMOUS)
	#[arg(unit = kilobyte)]
	Shmem: Information,
	/// (since Linux 4.20)
	/// Kernel allocations that the kernel will attempt to reclaim under memory
	/// pressure. Includes [`SReclaimable`](Self::SReclaimable), and other
	/// direct allocations with a shrinker.
	#[arg(unit = kilobyte)]
	KReclaimable: Information,
	/// In-kernel data structures cache.
	#[arg(unit = kilobyte)]
	Slab: Information,
	/// (since Linux 2.6.19)
	/// Part of [`Slab`](Self::Slab), that might be reclaimed, such as caches.
	#[arg(unit = kilobyte)]
	SReclaimable: Information,
	/// (since Linux 2.6.19)
	/// Part of [`Slab`](Self::Slab), that cannot be reclaimed on memory
	/// pressure.
	#[arg(unit = kilobyte)]
	SUnreclaim: Information,
	/// (since Linux 2.6.32)
	/// Amount of memory allocated to kernel stacks.
	#[arg(unit = kilobyte)]
	KernelStack: Information,
	/// (since Linux 2.6.18)
	/// Amount of memory dedicated to the lowest level of page tables.
	#[arg(unit = kilobyte)]
	PageTables: Information,
	// TODO: SecPageTables is ignored because it don't exist on docs.
	/// (since Linux 2.6.18)
	/// NFS pages sent to the server, but not yet committed to stable storage.
	#[arg(key = "NFS_Unstable", unit = kilobyte)]
	NFS_Unstable: Information,
	/// (since Linux 2.6.18)
	/// Memory used for block device "bounce buffers".
	#[arg(unit = kilobyte)]
	Bounce: Information,
	/// (since Linux 2.6.26)
	/// Memory used by FUSE for temporary writeback buffers.
	#[arg(unit = kilobyte)]
	WritebackTmp: Information,
	/// (since Linux 2.6.10)
	/// This is the total amount of memory currently available to be allocated
	/// on the system, expressed in kilobytes. This limit is adhered to only
	/// if strict overcommit accounting is enabled (mode 2 in
	/// /proc/sys/vm/overcommit_memory). The limit is calculated according to
	/// the formula described under /proc/sys/vm/overcommit_memory. For further
	/// details, see the kernel source file
	///  Documentation/vm/overcommit-accounting.rst.
	#[arg(unit = kilobyte)]
	CommitLimit: Information,
	/// The amount of memory presently allocated on the system. The committed
	/// memory is a sum of all of the memory which has been allocated by
	/// processes, even if it has not been "used" by them as of yet. A process
	/// which allocates 1 GB of memory (using malloc(3) or similar), but
	/// touches only 300 MB of that memory will show up as using only 300 MB of
	/// memory even if it has the address space allocated for the entire 1 GB.
	/// This 1 GB is memory which has been "committed" to by the VM and can be
	/// used at any time by the allocating application.  With strict overcommit
	/// enabled on the system (mode 2 in /proc/sys/vm/overcommit_memory),
	/// allocations which would exceed the CommitLimit will not be permitted.
	/// This is useful if one needs to guarantee that processes will not fail
	/// due to lack of memory once that memory has been successfully allocated.
	#[arg(key = "Committed_AS", unit = kilobyte)]
	Committed_AS: Information,
	/// Total size of vmalloc memory area.
	#[arg(unit = kilobyte)]
	VmallocTotal: Information,
	/// Amount of vmalloc area which is used.  Since Linux 4.4, this field is
	/// no longer calculated, and is hard coded as 0.  See /proc/vmallocinfo.
	#[arg(unit = kilobyte)]
	VmallocUsed: Information,
	/// Largest contiguous block of vmalloc area which is free.  Since Linux
	/// 4.4, this field is no longer calculated and is hard coded as 0. See
	/// /proc/vmallocinfo.
	#[arg(unit = kilobyte)]
	VmallocChunk: Information,
	// TODO: Percpu is ignored because it don't exist on docs.
}

pub async fn meminfo() -> Result<MemInfo, MemoryMetricError> {
	let content = read_to_string(procfs_root().join("meminfo")).await?;
	MemInfo::parse(&content).map_err(|e| MemoryMetricError::ParseError(Box::new(e)))
}
