use fake::{Dummy, Fake, Faker};
use std::fmt;

#[derive(Debug, Dummy)]
pub struct FakeVmStat {
	/// (since Linux 2.6.31)
	pub nr_free_pages: u64,
	/// (since Linux 2.6.28)
	pub nr_inactive_anon: u64,
	/// (since Linux 2.6.28)
	pub nr_active_anon: u64,
	/// (since Linux 2.6.28)
	pub nr_inactive_file: u64,
	/// (since Linux 2.6.28)
	pub nr_active_file: u64,
	/// (since Linux 2.6.28)
	pub nr_unevictable: u64,
	/// (since Linux 2.6.28)
	pub nr_mlock: u64,
	/// (since Linux 2.6.28)
	pub nr_anon_pages: u64,
	/// (since Linux 2.6.0)
	pub nr_mapped: u64,
	/// (since Linux 2.6.18)
	pub nr_file_pages: u64,
	/// (since Linux 2.6.0)
	pub nr_dirty: u64,
	/// (since Linux 2.6.0)
	pub nr_writeback: u64,
	/// (since Linux 2.6.19)
	pub nr_slab_reclaimable: u64,
	/// (since Linux 2.6.19)
	pub nr_slab_unreclaimable: u64,
	/// (since Linux 2.6.0)
	pub nr_page_table_pages: u64,
	/// (since Linux 2.6.32)
	/// Amount of memory allocated to kernel stacks.
	pub nr_kernel_stack: u64,
	/// (since Linux 2.6.0)
	pub nr_unstable: u64,
	/// (since Linux 2.6.12)
	pub nr_bounce: u64,
	/// (since Linux 2.6.19)
	pub nr_vmscan_write: u64,
	/// (since Linux 3.2)
	pub nr_vmscan_immediate_reclaim: u64,
	/// (since Linux 2.6.26)
	pub nr_writeback_temp: u64,
	/// (since Linux 2.6.32)
	pub nr_isolated_anon: u64,
	/// (since Linux 2.6.32)
	pub nr_isolated_file: u64,
	/// (since Linux 2.6.32)
	/// Pages used by shmem and tmpfs
	pub nr_shmem: u64,
	/// (since Linux 2.6.37)
	pub nr_dirtied: u64,
	/// (since Linux 2.6.37)
	pub nr_written: u64,
	/// (since Linux 2.6.18)
	pub numa_hit: u64,
	/// (since Linux 2.6.18)
	pub numa_miss: u64,
	/// (since Linux 2.6.18)
	pub numa_foreign: u64,
	/// (since Linux 2.6.18)
	pub numa_interleave: u64,
	/// (since Linux 2.6.18)
	pub numa_local: u64,
	/// (since Linux 2.6.18)
	pub numa_other: u64,
	/// (since Linux 3.15)
	pub workingset_nodereclaim: u64,
	/// (since Linux 2.6.38)
	pub nr_anon_transparent_hugepages: u64,
	/// (since Linux 3.7)
	/// Number of free CMA (Contiguous Memory Allocator) pages.
	pub nr_free_cma: u64,
	/// (since Linux 2.6.37)
	pub nr_dirty_threshold: u64,
	/// (since Linux 2.6.37)
	pub nr_dirty_background_threshold: u64,
	/// (since Linux 2.6.0)
	pub pgpgin: u64,
	/// (since Linux 2.6.0)
	pub pgpgout: u64,
	/// (since Linux 2.6.0)
	pub pswpin: u64,
	/// (since Linux 2.6.0)
	pub pswpout: u64,
	/// (since Linux 2.6.5)
	pub pgalloc_dma: u64,
	/// (since Linux 2.6.16)
	pub pgalloc_dma32: u64,
	/// (since Linux 2.6.5)
	pub pgalloc_normal: u64,
	/// (since Linux 2.6.23)
	pub pgalloc_movable: u64,
	/// (since Linux 2.6.0)
	pub pgfree: u64,
	/// (since Linux 2.6.0)
	pub pgactivate: u64,
	/// (since Linux 2.6.0)
	pub pgdeactivate: u64,
	/// (since Linux 2.6.0)
	pub pgfault: u64,
	/// (since Linux 2.6.0)
	pub pgmajfault: u64,
	/// (since Linux 3.6)
	pub pgscan_direct_throttle: u64,
	/// (since Linux 2.6.0)
	pub pginodesteal: u64,
	/// (since Linux 2.6.5)
	pub slabs_scanned: u64,
	/// (since Linux 2.6.0)
	pub kswapd_inodesteal: u64,
	/// (since Linux 2.6.33)
	pub kswapd_low_wmark_hit_quickly: u64,
	/// (since Linux 2.6.33)
	pub kswapd_high_wmark_hit_quickly: u64,
	/// (since Linux 2.6.0)
	pub pageoutrun: u64,
	/// (since Linux 2.6.0)
	pub pgrotated: u64,
	/// (since Linux 3.15)
	pub drop_pagecache: u64,
	/// (since Linux 3.15)
	pub drop_slab: u64,
	/// (since Linux 3.8)
	pub numa_pte_updates: u64,
	/// (since Linux 3.13)
	pub numa_huge_pte_updates: u64,
	/// (since Linux 3.8)
	pub numa_hint_faults: u64,
	/// (since Linux 3.8)
	pub numa_hint_faults_local: u64,
	/// (since Linux 3.8)
	pub numa_pages_migrated: u64,
	/// (since Linux 3.8)
	pub pgmigrate_success: u64,
	/// (since Linux 3.8)
	pub pgmigrate_fail: u64,
	/// (since Linux 3.8)
	pub compact_migrate_scanned: u64,
	/// (since Linux 3.8)
	pub compact_free_scanned: u64,
	/// (since Linux 3.8)
	pub compact_isolated: u64,
	/// (since Linux 2.6.35)
	pub compact_stall: u64,
	/// (since Linux 2.6.35)
	pub compact_fail: u64,
	/// (since Linux 2.6.35)
	pub compact_success: u64,
	/// (since Linux 2.6.26)
	pub htlb_buddy_alloc_success: u64,
	/// (since Linux 2.6.26)
	pub htlb_buddy_alloc_fail: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_culled: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_scanned: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_rescued: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_mlocked: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_munlocked: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_cleared: u64,
	/// (since Linux 2.6.28)
	pub unevictable_pgs_stranded: u64,
	/// (since Linux 2.6.39)
	pub thp_fault_alloc: u64,
	/// (since Linux 2.6.39)
	pub thp_fault_fallback: u64,
	/// (since Linux 2.6.39)
	pub thp_collapse_alloc: u64,
	/// (since Linux 2.6.39)
	pub thp_collapse_alloc_failed: u64,
	/// (since Linux 2.6.39)
	pub thp_zero_page_alloc: u64,
	/// (since Linux 2.6.39)
	pub thp_zero_page_alloc_failed: u64,
	/// (since Linux 3.18)
	pub balloon_inflate: u64,
	/// (since Linux 3.18)
	pub balloon_deflate: u64,
	/// (since Linux 3.18)
	pub balloon_migrate: u64,
}

impl FakeVmStat {
	pub fn generate() -> Self {
		Faker.fake()
	}
}

impl fmt::Display for FakeVmStat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"nr_free_pages {}
nr_zone_inactive_anon {}
nr_zone_active_anon {}
nr_zone_inactive_file {}
nr_zone_active_file {}
nr_zone_unevictable {}
nr_zone_write_pending {}
nr_mlock {}
nr_bounce {}
nr_zspages {}
nr_free_cma {}
nr_unaccepted {}
numa_hit {}
numa_miss {}
numa_foreign {}
numa_interleave {}
numa_local {}
numa_other {}
nr_inactive_anon {}
nr_active_anon {}
nr_inactive_file {}
nr_active_file {}
nr_unevictable {}
nr_slab_reclaimable {}
nr_slab_unreclaimable {}
nr_isolated_anon {}
nr_isolated_file {}
workingset_nodes {}
workingset_refault_anon {}
workingset_refault_file {}
workingset_activate_anon {}
workingset_activate_file {}
workingset_restore_anon {}
workingset_restore_file {}
workingset_nodereclaim {}
nr_anon_pages {}
nr_mapped {}
nr_file_pages {}
nr_dirty {}
nr_writeback {}
nr_writeback_temp {}
nr_shmem {}
nr_shmem_hugepages {}
nr_shmem_pmdmapped {}
nr_file_hugepages {}
nr_file_pmdmapped {}
nr_anon_transparent_hugepages {}
nr_vmscan_write {}
nr_vmscan_immediate_reclaim {}
nr_dirtied {}
nr_written {}
nr_throttled_written {}
nr_kernel_misc_reclaimable {}
nr_foll_pin_acquired {}
nr_foll_pin_released {}
nr_kernel_stack {}
nr_page_table_pages {}
nr_sec_page_table_pages {}
nr_swapcached {}
pgpromote_success {}
pgpromote_candidate {}
pgdemote_kswapd {}
pgdemote_direct {}
pgdemote_khugepaged {}
nr_dirty_threshold {}
nr_dirty_background_threshold {}
pgpgin {}
pgpgout {}
pswpin {}
pswpout {}
pgalloc_dma {}
pgalloc_dma32 {}
pgalloc_normal {}
pgalloc_movable {}
pgalloc_device {}
allocstall_dma {}
allocstall_dma32 {}
allocstall_normal {}
allocstall_movable {}
allocstall_device {}
pgskip_dma {}
pgskip_dma32 {}
pgskip_normal {}
pgskip_movable {}
pgskip_device {}
pgfree {}
pgactivate {}
pgdeactivate {}
pglazyfree {}
pgfault {}
pgmajfault {}
pglazyfreed {}
pgrefill {}
pgreuse {}
pgsteal_kswapd {}
pgsteal_direct {}
pgsteal_khugepaged {}
pgscan_kswapd {}
pgscan_direct {}
pgscan_khugepaged {}
pgscan_direct_throttle {}
pgscan_anon {}
pgscan_file {}
pgsteal_anon {}
pgsteal_file {}
zone_reclaim_failed {}
pginodesteal {}
slabs_scanned {}
kswapd_inodesteal {}
kswapd_low_wmark_hit_quickly {}
kswapd_high_wmark_hit_quickly {}
pageoutrun {}
pgrotated {}
drop_pagecache {}
drop_slab {}
oom_kill {}
numa_pte_updates {}
numa_huge_pte_updates {}
numa_hint_faults {}
numa_hint_faults_local {}
numa_pages_migrated {}
pgmigrate_success {}
pgmigrate_fail {}
thp_migration_success {}
thp_migration_fail {}
thp_migration_split {}
compact_migrate_scanned {}
compact_free_scanned {}
compact_isolated {}
compact_stall {}
compact_fail {}
compact_success {}
compact_daemon_wake {}
compact_daemon_migrate_scanned {}
compact_daemon_free_scanned {}
htlb_buddy_alloc_success {}
htlb_buddy_alloc_fail {}
unevictable_pgs_culled {}
unevictable_pgs_scanned {}
unevictable_pgs_rescued {}
unevictable_pgs_mlocked {}
unevictable_pgs_munlocked {}
unevictable_pgs_cleared {}
unevictable_pgs_stranded {}
thp_fault_alloc {}
thp_fault_fallback {}
thp_fault_fallback_charge {}
thp_collapse_alloc {}
thp_collapse_alloc_failed {}
thp_file_alloc {}
thp_file_fallback {}
thp_file_fallback_charge {}
thp_file_mapped {}
thp_split_page {}
thp_split_page_failed {}
thp_deferred_split_page {}
thp_split_pmd {}
thp_scan_exceed_none_pte {}
thp_scan_exceed_swap_pte {}
thp_scan_exceed_share_pte {}
thp_split_pud {}
thp_zero_page_alloc {}
thp_zero_page_alloc_failed {}
thp_swpout {}
thp_swpout_fallback {}
balloon_inflate {}
balloon_deflate {}
balloon_migrate {}
swap_ra {}
swap_ra_hit {}
ksm_swpin_copy {}
cow_ksm {}
zswpin {}
zswpout {}
zswpwb {}
direct_map_level2_splits {}
direct_map_level3_splits {}
nr_unstable {}",
			self.nr_free_pages,
			0, // nr_zone_inactive_anon,
			0, // nr_zone_active_anon,
			0, // nr_zone_inactive_file,
			0, // nr_zone_active_file,
			0, // nr_zone_unevictable,
			0, // nr_zone_write_pending,
			self.nr_mlock,
			self.nr_bounce,
			0, // nr_zspages,
			self.nr_free_cma,
			0, // nr_unaccepted,
			self.numa_hit,
			self.numa_miss,
			self.numa_foreign,
			self.numa_interleave,
			self.numa_local,
			self.numa_other,
			self.nr_inactive_anon,
			self.nr_active_anon,
			self.nr_inactive_file,
			self.nr_active_file,
			self.nr_unevictable,
			self.nr_slab_reclaimable,
			self.nr_slab_unreclaimable,
			self.nr_isolated_anon,
			self.nr_isolated_file,
			0, // workingset_nodes,
			0, // workingset_refault_anon,
			0, // workingset_refault_file,
			0, // workingset_activate_anon,
			0, // workingset_activate_file,
			0, // workingset_restore_anon,
			0, // workingset_restore_file,
			self.workingset_nodereclaim,
			self.nr_anon_pages,
			self.nr_mapped,
			self.nr_file_pages,
			self.nr_dirty,
			self.nr_writeback,
			self.nr_writeback_temp,
			self.nr_shmem,
			0, // nr_shmem_hugepages,
			0, // nr_shmem_pmdmapped,
			0, // nr_file_hugepages,
			0, // nr_file_pmdmapped,
			self.nr_anon_transparent_hugepages,
			self.nr_vmscan_write,
			self.nr_vmscan_immediate_reclaim,
			self.nr_dirtied,
			self.nr_written,
			0, // nr_throttled_written,
			0, // nr_kernel_misc_reclaimable,
			0, // nr_foll_pin_acquired,
			0, // nr_foll_pin_released,
			self.nr_kernel_stack,
			self.nr_page_table_pages,
			0, // nr_sec_page_table_pages,
			0, // nr_swapcached,
			0, // pgpromote_success,
			0, // pgpromote_candidate,
			0, // pgdemote_kswapd,
			0, // pgdemote_direct,
			0, // pgdemote_khugepaged,
			self.nr_dirty_threshold,
			self.nr_dirty_background_threshold,
			self.pgpgin,
			self.pgpgout,
			self.pswpin,
			self.pswpout,
			self.pgalloc_dma,
			self.pgalloc_dma32,
			self.pgalloc_normal,
			self.pgalloc_movable,
			0, // pgalloc_device,
			0, // allocstall_dma,
			0, // allocstall_dma32,
			0, // allocstall_normal,
			0, // allocstall_movable,
			0, // allocstall_device,
			0, // pgskip_dma,
			0, // pgskip_dma32,
			0, // pgskip_normal,
			0, // pgskip_movable,
			0, // pgskip_device,
			self.pgfree,
			self.pgactivate,
			self.pgdeactivate,
			0, // pglazyfree,
			self.pgfault,
			self.pgmajfault,
			0, // pglazyfreed,
			0, // pgrefill,
			0, // pgreuse,
			0, // pgsteal_kswapd,
			0, // pgsteal_direct,
			0, // pgsteal_khugepaged,
			0, // pgscan_kswapd,
			0, // pgscan_direct,
			0, // pgscan_khugepaged,
			self.pgscan_direct_throttle,
			0, // pgscan_anon,
			0, // pgscan_file,
			0, // pgsteal_anon,
			0, // pgsteal_file,
			0, // zone_reclaim_failed,
			self.pginodesteal,
			self.slabs_scanned,
			self.kswapd_inodesteal,
			self.kswapd_low_wmark_hit_quickly,
			self.kswapd_high_wmark_hit_quickly,
			self.pageoutrun,
			self.pgrotated,
			self.drop_pagecache,
			self.drop_slab,
			0, // oom_kill,
			self.numa_pte_updates,
			self.numa_huge_pte_updates,
			self.numa_hint_faults,
			self.numa_hint_faults_local,
			self.numa_pages_migrated,
			self.pgmigrate_success,
			self.pgmigrate_fail,
			0, // thp_migration_success,
			0, // thp_migration_fail
			0, // thp_migration_split
			self.compact_migrate_scanned,
			self.compact_free_scanned,
			self.compact_isolated,
			self.compact_stall,
			self.compact_fail,
			self.compact_success,
			0, // compact_daemon_wake
			0, // compact_daemon_migrate_scanned
			0, // compact_daemon_free_scanned
			self.htlb_buddy_alloc_success,
			self.htlb_buddy_alloc_fail,
			self.unevictable_pgs_culled,
			self.unevictable_pgs_scanned,
			self.unevictable_pgs_rescued,
			self.unevictable_pgs_mlocked,
			self.unevictable_pgs_munlocked,
			self.unevictable_pgs_cleared,
			self.unevictable_pgs_stranded,
			self.thp_fault_alloc,
			self.thp_fault_fallback,
			0, // thp_file_fallback_charge
			self.thp_collapse_alloc,
			self.thp_collapse_alloc_failed,
			0, // thp_file_alloc
			0, // thp_file_fallback
			0, // thp_file_fallback_charge
			0, // thp_file_mapped
			0, // thp_split_page
			0, // thp_split_page_failed
			0, // thp_deferred_split_page
			0, // thp_split_pmd
			0, // thp_scan_exceed_none_pte
			0, // thp_scan_exceed_swap_pte
			0, // thp_scan_exceed_share_pte
			0, // thp_split_pud
			self.thp_zero_page_alloc,
			self.thp_zero_page_alloc_failed,
			0, // thp_swpout
			0, // thp_swpout_fallback
			self.balloon_inflate,
			self.balloon_deflate,
			self.balloon_migrate,
			0, // swap_ra
			0, // swap_ra_hit
			0, // ksm_swpin_copy
			0, // cow_ksm
			0, // zswpin
			0, // zswpout
			0, // zswpwb
			0, // direct_map_level2_splits
			0, // direct_map_level3_splits
			self.nr_unstable,
		)?;
		Ok(())
	}
}
