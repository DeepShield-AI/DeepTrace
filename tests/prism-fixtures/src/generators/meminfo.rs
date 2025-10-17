#![allow(non_snake_case)]

use fake::{Dummy, Fake, Faker};
use std::fmt;

#[derive(Debug, Dummy)]
pub struct FakeMemInfo {
	/// Total usable RAM (i.e., physical RAM minus a few reserved bits and the
	/// kernel binary code).
	pub MemTotal: u64,
	/// The sum of [`LowFree`] + [`HighFree`]
	pub MemFree: u64,
	/// (since Linux 3.14)
	/// An estimate of how much memory is available for starting new
	/// applications, without swapping.
	pub MemAvailable: u64,
	/// Relatively temporary storage for raw disk blocks that shouldn't get
	/// tremendously large (20 MB or so)
	pub Buffers: u64,
	/// In-memory cache for files read from the disk (the page cache). Doesn't
	/// include [`SwapCached`](fake::SwapCached).
	pub Cached: u64,
	/// Memory that once was swapped out, is swapped back in but still also is
	/// in the swap file. (If memory pressure is high, these pages don't need
	/// to be swapped out again because they are already in the swap file. This
	/// saves I/O.)
	pub SwapCached: u64,
	/// Memory that has been used more recently and usually not reclaimed
	/// unless absolutely necessary.
	pub Active: u64,
	/// Memory which has been less recently used. It is more eligible to be
	/// reclaimed for other purposes.
	pub Inactive: u64,
	/// (since Linux 2.6.28)
	pub Active_anon: u64,
	/// (since Linux 2.6.28)
	pub Inactive_anon: u64,
	/// (since Linux 2.6.28)
	pub Active_file: u64,
	/// (since Linux 2.6.28)
	pub Inactive_file: u64,
	/// Total amount of swap space available.
	pub SwapTotal: u64,
	/// Amount of swap space that is currently unused.
	pub SwapFree: u64,
	/// Memory which is waiting to get written back to the disk.
	pub Dirty: u64,
	/// Writeback
	pub Writeback: u64,
	/// (since Linux 2.6.18)
	/// Non-file backed pages mapped into user-space page tables.
	pub AnonPages: u64,
	/// Files which have been mapped into memory (with mmap(2)), such as
	/// libraries.
	pub Mapped: u64,
	/// (since Linux 2.6.32)
	/// Amount of memory consumed in tmpfs(5) filesystems, System V, and POSIX
	/// shared memory, as well as shared anonymous mappings
	/// (MAP_SHARED|MAP_ANONYMOUS)
	pub Shmem: u64,
	/// (since Linux 4.20)
	/// Kernel allocations that the kernel will attempt to reclaim under memory
	/// pressure. Includes [`SReclaimable`](fake::SReclaimable), and other
	/// direct allocations with a shrinker.
	pub KReclaimable: u64,
	/// In-kernel data structures cache.
	pub Slab: u64,
	/// (since Linux 2.6.19)
	/// Part of [`Slab`](fake::Slab), that might be reclaimed, such as caches.
	pub SReclaimable: u64,
	/// (since Linux 2.6.19)
	/// Part of [`Slab`](fake::Slab), that cannot be reclaimed on memory
	/// pressure.
	pub SUnreclaim: u64,
	/// (since Linux 2.6.32)
	/// Amount of memory allocated to kernel stacks.
	pub KernelStack: u64,
	/// (since Linux 2.6.18)
	/// Amount of memory dedicated to the lowest level of page tables.
	pub PageTables: u64,
	/// (since Linux 2.6.18)
	/// NFS pages sent to the server, but not yet committed to stable storage.
	#[dummy(faker = "1..1_000_000_000")]
	pub NFS_Unstable: u64,
	/// (since Linux 2.6.18)
	/// Memory used for block device "bounce buffers".
	pub Bounce: u64,
	/// (since Linux 2.6.26)
	/// Memory used by FUSE for temporary writeback buffers.
	pub WritebackTmp: u64,
	/// (since Linux 2.6.10)
	/// This is the total amount of memory currently available to be allocated
	/// on the system, expressed in kilobytes. This limit is adhered to only
	/// if strict overcommit accounting is enabled (mode 2 in
	/// /proc/sys/vm/overcommit_memory). The limit is calculated according to
	/// the formula described under /proc/sys/vm/overcommit_memory. For further
	/// details, see the kernel source file
	///  Documentation/vm/overcommit-accounting.rst.
	pub CommitLimit: u64,
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
	pub Committed_AS: u64,
	/// Total size of vmalloc memory area.
	pub VmallocTotal: u64,
	/// Amount of vmalloc area which is used.  Since Linux 4.4, this field is
	/// no longer calculated, and is hard coded as 0.  See /proc/vmallocinfo.
	pub VmallocUsed: u64,
	/// Largest contiguous block of vmalloc area which is free.  Since Linux
	/// 4.4, this field is no longer calculated and is hard coded as 0. See
	/// /proc/vmallocinfo.
	pub VmallocChunk: u64,
}

impl FakeMemInfo {
	pub fn generate() -> Self {
		Faker.fake()
	}
}

impl fmt::Display for FakeMemInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"MemTotal:{:>15} kB
MemFree:{:>16} kB
MemAvailable:{:>11} kB
Buffers:{:>16} kB
Cached:{:>17} kB
SwapCached:{:>13} kB
Active:{:>17} kB
Inactive:{:>15} kB
Active(anon):{:>11} kB
Inactive(anon):{:>9} kB
Active(file):{:>11} kB
Inactive(file):{:>9} kB
Unevictable:{:>11} kB
Mlocked:{:>16} kB
SwapTotal:{:>14} kB
SwapFree:{:>15} kB
Zswap:{:>18} kB
Zswapped:{:>15} kB
Dirty:{:>20} kB
Writeback:{:>14} kB
AnonPages:{:>14} kB
Mapped:{:>17} kB
Shmem:{:>18} kB
KReclaimable:{:>11} kB
Slab:{:>19} kB
SReclaimable:{:>11} kB
SUnreclaim:{:>13} kB
KernelStack:{:>12} kB
PageTables:{:>13} kB
SecPageTables:{:>10} kB
NFS_Unstable:{:>11} kB
Bounce:{:>17} kB
WritebackTmp:{:>11} kB
CommitLimit:{:>12} kB
Committed_AS:{:>11} kB
VmallocTotal:{:>11} kB
VmallocUsed:{:>12} kB
VmallocChunk:{:>11} kB
Percpu:{:>17} kB
HardwareCorrupted:{:>6} kB
AnonHugePages:{:>10} kB
ShmemHugePages:{:>9} kB
ShmemPmdMapped:{:>9} kB
FileHugePages:{:>10} kB
FilePmdMapped:{:>10} kB
Unaccepted:{:>13} kB
HugePages_Total:{:>8} kB
HugePages_Free:{:>9} kB
HugePages_Rsvd:{:>9} kB
HugePages_Surp:{:>9} kB
Hugepagesize:{:>11} kB
Hugetlb:{:>16} kB
DirectMap4k:{:>12} kB
DirectMap2M:{:>12} kB
DirectMap1G:{:>12} kB
",
			self.MemTotal,
			self.MemFree,
			self.MemAvailable,
			self.Buffers,
			self.Cached,
			self.SwapCached,
			self.Active,
			self.Inactive,
			self.Active_anon,
			self.Inactive_anon,
			self.Active_file,
			self.Inactive_file,
			0, // Unevictable
			0, // Mlocked
			self.SwapTotal,
			self.SwapFree,
			0, // Zswap
			0, // Zswapped
			self.Dirty,
			self.Writeback,
			self.AnonPages,
			self.Mapped,
			self.Shmem,
			self.KReclaimable,
			self.Slab,
			self.SReclaimable,
			self.SUnreclaim,
			self.KernelStack,
			self.PageTables,
			0, // SecPageTables
			self.NFS_Unstable,
			self.Bounce,
			self.WritebackTmp,
			self.CommitLimit,
			self.Committed_AS,
			self.VmallocTotal,
			self.VmallocUsed,
			self.VmallocChunk,
			0, // Percpu
			0, // HardwareCorrupted
			0, // AnonHugePages
			0, // ShmemHugePages
			0, // ShmemPmdMapped
			0, // FileHugePages
			0, // FilePmdMapped
			0, // Unaccepted
			0, // HugePages_Total
			0, // HugePages_Free
			0, // HugePages_Rsvd
			0, // HugePages_Surp
			0, // Hugepagesize
			0, // Hugetlb
			0, // DirectMap4k
			0, // DirectMap2M
			0, // DirectMap1G
		)?;
		Ok(())
	}
}
