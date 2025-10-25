//! virtual memory statistics
//!
//! This file displays various virtual memory statistics. Each line of this
//! file contains a single name-value pair, delimited by white space. Some
//! lines are present only if the kernel was configured with suitable options.
//! (In some cases, the options required for particular files have changed
//! across kernel versions, so they are not listed here. Details can be found
//! by consulting the kernel source code.)

#![allow(non_snake_case)]

use crate::MemoryMetricError;
use observ_macros::ProcParser;
use observ_metric_common::procfs_root;
use observ_metric_utils::read_to_string;

#[derive(ProcParser)]
#[fmt = "space"]
pub struct VmStat {
	/// (since Linux 2.6.31)
	nr_free_pages: u64,
	/// (since Linux 2.6.28)
	nr_inactive_anon: u64,
	/// (since Linux 2.6.28)
	nr_active_anon: u64,
	/// (since Linux 2.6.28)
	nr_inactive_file: u64,
	/// (since Linux 2.6.28)
	nr_active_file: u64,
	/// (since Linux 2.6.28)
	nr_unevictable: u64,
	/// (since Linux 2.6.28)
	nr_mlock: u64,
	/// (since Linux 2.6.28)
	nr_anon_pages: u64,
	/// (since Linux 2.6.0)
	nr_mapped: u64,
	/// (since Linux 2.6.18)
	nr_file_pages: u64,
	/// (since Linux 2.6.0)
	nr_dirty: u64,
	/// (since Linux 2.6.0)
	nr_writeback: u64,
	/// (since Linux 2.6.19)
	nr_slab_reclaimable: u64,
	/// (since Linux 2.6.19)
	nr_slab_unreclaimable: u64,
	/// (since Linux 2.6.0)
	nr_page_table_pages: u64,
	/// (since Linux 2.6.32)
	/// Amount of memory allocated to kernel stacks.
	nr_kernel_stack: u64,
	/// (since Linux 2.6.0)
	nr_unstable: u64,
	/// (since Linux 2.6.12)
	nr_bounce: u64,
	/// (since Linux 2.6.19)
	nr_vmscan_write: u64,
	/// (since Linux 3.2)
	nr_vmscan_immediate_reclaim: u64,
	/// (since Linux 2.6.26)
	nr_writeback_temp: u64,
	/// (since Linux 2.6.32)
	nr_isolated_anon: u64,
	/// (since Linux 2.6.32)
	nr_isolated_file: u64,
	/// (since Linux 2.6.32)
	/// Pages used by shmem and tmpfs
	nr_shmem: u64,
	/// (since Linux 2.6.37)
	nr_dirtied: u64,
	/// (since Linux 2.6.37)
	nr_written: u64,
	/// (since Linux 2.6.18)
	numa_hit: u64,
	/// (since Linux 2.6.18)
	numa_miss: u64,
	/// (since Linux 2.6.18)
	numa_foreign: u64,
	/// (since Linux 2.6.18)
	numa_interleave: u64,
	/// (since Linux 2.6.18)
	numa_local: u64,
	/// (since Linux 2.6.18)
	numa_other: u64,
	/// (since Linux 3.15)
	workingset_nodereclaim: u64,
	/// (since Linux 2.6.38)
	nr_anon_transparent_hugepages: u64,
	/// (since Linux 3.7)
	/// Number of free CMA (Contiguous Memory Allocator) pages.
	nr_free_cma: u64,
	/// (since Linux 2.6.37)
	nr_dirty_threshold: u64,
	/// (since Linux 2.6.37)
	nr_dirty_background_threshold: u64,
	/// (since Linux 2.6.0)
	pgpgin: u64,
	/// (since Linux 2.6.0)
	pgpgout: u64,
	/// (since Linux 2.6.0)
	pswpin: u64,
	/// (since Linux 2.6.0)
	pswpout: u64,
	/// (since Linux 2.6.5)
	pgalloc_dma: u64,
	/// (since Linux 2.6.16)
	pgalloc_dma32: u64,
	/// (since Linux 2.6.5)
	pgalloc_normal: u64,
	/// (since Linux 2.6.23)
	pgalloc_movable: u64,
	/// (since Linux 2.6.0)
	pgfree: u64,
	/// (since Linux 2.6.0)
	pgactivate: u64,
	/// (since Linux 2.6.0)
	pgdeactivate: u64,
	/// (since Linux 2.6.0)
	pgfault: u64,
	/// (since Linux 2.6.0)
	pgmajfault: u64,
	/// (since Linux 3.6)
	pgscan_direct_throttle: u64,
	/// (since Linux 2.6.0)
	pginodesteal: u64,
	/// (since Linux 2.6.5)
	slabs_scanned: u64,
	/// (since Linux 2.6.0)
	kswapd_inodesteal: u64,
	/// (since Linux 2.6.33)
	kswapd_low_wmark_hit_quickly: u64,
	/// (since Linux 2.6.33)
	kswapd_high_wmark_hit_quickly: u64,
	/// (since Linux 2.6.0)
	pageoutrun: u64,
	/// (since Linux 2.6.0)
	pgrotated: u64,
	/// (since Linux 3.15)
	drop_pagecache: u64,
	/// (since Linux 3.15)
	drop_slab: u64,
	/// (since Linux 3.8)
	numa_pte_updates: u64,
	/// (since Linux 3.13)
	numa_huge_pte_updates: u64,
	/// (since Linux 3.8)
	numa_hint_faults: u64,
	/// (since Linux 3.8)
	numa_hint_faults_local: u64,
	/// (since Linux 3.8)
	numa_pages_migrated: u64,
	/// (since Linux 3.8)
	pgmigrate_success: u64,
	/// (since Linux 3.8)
	pgmigrate_fail: u64,
	/// (since Linux 3.8)
	compact_migrate_scanned: u64,
	/// (since Linux 3.8)
	compact_free_scanned: u64,
	/// (since Linux 3.8)
	compact_isolated: u64,
	/// (since Linux 2.6.35)
	compact_stall: u64,
	/// (since Linux 2.6.35)
	compact_fail: u64,
	/// (since Linux 2.6.35)
	compact_success: u64,
	/// (since Linux 2.6.26)
	htlb_buddy_alloc_success: u64,
	/// (since Linux 2.6.26)
	htlb_buddy_alloc_fail: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_culled: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_scanned: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_rescued: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_mlocked: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_munlocked: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_cleared: u64,
	/// (since Linux 2.6.28)
	unevictable_pgs_stranded: u64,
	/// (since Linux 2.6.39)
	thp_fault_alloc: u64,
	/// (since Linux 2.6.39)
	thp_fault_fallback: u64,
	/// (since Linux 2.6.39)
	thp_collapse_alloc: u64,
	/// (since Linux 2.6.39)
	thp_collapse_alloc_failed: u64,
	/// (since Linux 2.6.39)
	thp_zero_page_alloc: u64,
	/// (since Linux 2.6.39)
	thp_zero_page_alloc_failed: u64,
	/// (since Linux 3.18)
	balloon_inflate: u64,
	/// (since Linux 3.18)
	balloon_deflate: u64,
	/// (since Linux 3.18)
	balloon_migrate: u64,
}

pub async fn vmstat() -> Result<VmStat, MemoryMetricError> {
	let content = read_to_string(procfs_root().join("vmstat")).await?;
	VmStat::parse(&content).map_err(Into::into)
}
