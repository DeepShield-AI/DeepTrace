use super::{Collector, MetricError, constants::*};
use log::warn;
use observ_event::{gauge, metric::Metric};
use observ_memory::*;
use std::io;
use uom::si::information::kilobyte;

pub struct MemoryCollector;

impl MemoryCollector {
	pub(crate) const fn new() -> Result<Self, MetricError> {
		Ok(Self {})
	}
}

#[async_trait::async_trait]
impl Collector for MemoryCollector {
	fn name(&self) -> &'static str {
		"host memory collector"
	}

	async fn collect(&self, buffer: &mut Vec<Metric>) -> Result<(), MetricError> {
		match meminfo::meminfo().await {
			Ok(meminfo) => {
				buffer.push(gauge!(MEM_TOTAL, meminfo.get_MemTotal().get::<kilobyte>()));
				buffer.push(gauge!(MEM_FREE, meminfo.get_MemFree().get::<kilobyte>()));
				buffer.push(gauge!(MEM_AVAILABLE, meminfo.get_MemAvailable().get::<kilobyte>()));
				buffer.push(gauge!(BUFFERS, meminfo.get_Buffers().get::<kilobyte>()));
				buffer.push(gauge!(CACHED, meminfo.get_Cached().get::<kilobyte>()));
				buffer.push(gauge!(SWAP_CACHED, meminfo.get_SwapCached().get::<kilobyte>()));
				buffer.push(gauge!(ACTIVE, meminfo.get_Active().get::<kilobyte>()));
				buffer.push(gauge!(INACTIVE, meminfo.get_Inactive().get::<kilobyte>()));
				buffer.push(gauge!(ACTIVE_ANON, meminfo.get_Active_anon().get::<kilobyte>()));
				buffer.push(gauge!(INACTIVE_ANON, meminfo.get_Inactive_anon().get::<kilobyte>()));
				buffer.push(gauge!(ACTIVE_FILE, meminfo.get_Active_file().get::<kilobyte>()));
				buffer.push(gauge!(INACTIVE_FILE, meminfo.get_Inactive_file().get::<kilobyte>()));
				buffer.push(gauge!(SWAP_TOTAL, meminfo.get_SwapTotal().get::<kilobyte>()));
				buffer.push(gauge!(SWAP_FREE, meminfo.get_SwapFree().get::<kilobyte>()));
				buffer.push(gauge!(DIRTY, meminfo.get_Dirty().get::<kilobyte>()));
				buffer.push(gauge!(WRITEBACK, meminfo.get_Writeback().get::<kilobyte>()));
				buffer.push(gauge!(ANON_PAGES, meminfo.get_AnonPages().get::<kilobyte>()));
				buffer.push(gauge!(MAPPED, meminfo.get_Mapped().get::<kilobyte>()));
				buffer.push(gauge!(SHMEM, meminfo.get_Shmem().get::<kilobyte>()));
				buffer.push(gauge!(KRECLAIMABLE, meminfo.get_KReclaimable().get::<kilobyte>()));
				buffer.push(gauge!(SLAB, meminfo.get_Slab().get::<kilobyte>()));
				buffer.push(gauge!(SRECLAIMABLE, meminfo.get_SReclaimable().get::<kilobyte>()));
				buffer.push(gauge!(SUNRECLAIM, meminfo.get_SUnreclaim().get::<kilobyte>()));
				buffer.push(gauge!(KERNEL_STACK, meminfo.get_KernelStack().get::<kilobyte>()));
				buffer.push(gauge!(PAGE_TABLES, meminfo.get_PageTables().get::<kilobyte>()));
				buffer.push(gauge!(NFS_UNSTABLE, meminfo.get_NFS_Unstable().get::<kilobyte>()));
				buffer.push(gauge!(BOUNCE, meminfo.get_Bounce().get::<kilobyte>()));
				buffer.push(gauge!(WRITEBACK_TMP, meminfo.get_WritebackTmp().get::<kilobyte>()));
				buffer.push(gauge!(COMMIT_LIMIT, meminfo.get_CommitLimit().get::<kilobyte>()));
				buffer.push(gauge!(COMMITTED_AS, meminfo.get_Committed_AS().get::<kilobyte>()));
				buffer.push(gauge!(VMALLOC_TOTAL, meminfo.get_VmallocTotal().get::<kilobyte>()));
				buffer.push(gauge!(VMALLOC_USED, meminfo.get_VmallocUsed().get::<kilobyte>()));
				buffer.push(gauge!(VMALLOC_CHUNK, meminfo.get_VmallocChunk().get::<kilobyte>()));
			},
			Err(error) => {
				warn!("Failed to collect memory metrics: {error}");
				return Err(io::Error::last_os_error().into());
			},
		}
		match vmstat::vmstat().await {
			Ok(vmstat) => {
				buffer.push(gauge!(NR_FREE_PAGES, vmstat.get_nr_free_pages()));
				buffer.push(gauge!(NR_INACTIVE_ANON, vmstat.get_nr_inactive_anon()));
				buffer.push(gauge!(NR_ACTIVE_ANON, vmstat.get_nr_active_anon()));
				buffer.push(gauge!(NR_INACTIVE_FILE, vmstat.get_nr_inactive_file()));
				buffer.push(gauge!(NR_ACTIVE_FILE, vmstat.get_nr_active_file()));
				buffer.push(gauge!(NR_UNEVICTABLE, vmstat.get_nr_unevictable()));
				buffer.push(gauge!(NR_MLOCK, vmstat.get_nr_mlock()));
				buffer.push(gauge!(NR_ANON_PAGES, vmstat.get_nr_anon_pages()));
				buffer.push(gauge!(NR_MAPPED, vmstat.get_nr_mapped()));
				buffer.push(gauge!(NR_FILE_PAGES, vmstat.get_nr_file_pages()));
				buffer.push(gauge!(NR_DIRTY, vmstat.get_nr_dirty()));
				buffer.push(gauge!(NR_WRITEBACK, vmstat.get_nr_writeback()));
				buffer.push(gauge!(NR_SLAB_RECLAIMABLE, vmstat.get_nr_slab_reclaimable()));
				buffer.push(gauge!(NR_SLAB_UNRECLAIMABLE, vmstat.get_nr_slab_unreclaimable()));
				buffer.push(gauge!(NR_PAGE_TABLE_PAGES, vmstat.get_nr_page_table_pages()));
				buffer.push(gauge!(NR_KERNEL_STACK, vmstat.get_nr_kernel_stack()));
				buffer.push(gauge!(NR_UNSTABLE, vmstat.get_nr_unstable()));
				buffer.push(gauge!(NR_BOUNCE, vmstat.get_nr_bounce()));
				buffer.push(gauge!(NR_VMSCAN_WRITE, vmstat.get_nr_vmscan_write()));
				buffer.push(gauge!(
					NR_VMSCAN_IMMEDIATE_RECLAIM,
					vmstat.get_nr_vmscan_immediate_reclaim()
				));
				buffer.push(gauge!(NR_WRITEBACK_TEMP, vmstat.get_nr_writeback_temp()));
				buffer.push(gauge!(NR_ISOLATED_ANON, vmstat.get_nr_isolated_anon()));
				buffer.push(gauge!(NR_ISOLATED_FILE, vmstat.get_nr_isolated_file()));
				buffer.push(gauge!(NR_SHMEM, vmstat.get_nr_shmem()));
				buffer.push(gauge!(NR_DIRTIED, vmstat.get_nr_dirtied()));
				buffer.push(gauge!(NR_WRITTEN, vmstat.get_nr_written()));
				buffer.push(gauge!(NUMA_HIT, vmstat.get_numa_hit()));
				buffer.push(gauge!(NUMA_MISS, vmstat.get_numa_miss()));
				buffer.push(gauge!(NUMA_FOREIGN, vmstat.get_numa_foreign()));
				buffer.push(gauge!(NUMA_INTERLEAVE, vmstat.get_numa_interleave()));
				buffer.push(gauge!(NUMA_LOCAL, vmstat.get_numa_local()));
				buffer.push(gauge!(NUMA_OTHER, vmstat.get_numa_other()));
				buffer.push(gauge!(WORKINGSET_NODERECLAIM, vmstat.get_workingset_nodereclaim()));
				buffer.push(gauge!(
					NR_ANON_TRANSPARENT_HUGEPAGES,
					vmstat.get_nr_anon_transparent_hugepages()
				));
				buffer.push(gauge!(NR_FREE_CMA, vmstat.get_nr_free_cma()));
				buffer.push(gauge!(NR_DIRTY_THRESHOLD, vmstat.get_nr_dirty_threshold()));
				buffer.push(gauge!(
					NR_DIRTY_BACKGROUND_THRESHOLD,
					vmstat.get_nr_dirty_background_threshold()
				));
				buffer.push(gauge!(PGPGIN, vmstat.get_pgpgin()));
				buffer.push(gauge!(PGPGOUT, vmstat.get_pgpgout()));
				buffer.push(gauge!(PSWPIN, vmstat.get_pswpin()));
				buffer.push(gauge!(PSWPOUT, vmstat.get_pswpout()));
				buffer.push(gauge!(PGALLOC_DMA, vmstat.get_pgalloc_dma()));
				buffer.push(gauge!(PGALLOC_DMA32, vmstat.get_pgalloc_dma32()));
				buffer.push(gauge!(PGALLOC_NORMAL, vmstat.get_pgalloc_normal()));
				buffer.push(gauge!(PGALLOC_MOVABLE, vmstat.get_pgalloc_movable()));
				buffer.push(gauge!(PGFREE, vmstat.get_pgfree()));
				buffer.push(gauge!(PGACTIVATE, vmstat.get_pgactivate()));
				buffer.push(gauge!(PGDEACTIVATE, vmstat.get_pgdeactivate()));
				buffer.push(gauge!(PGFAULT, vmstat.get_pgfault()));
				buffer.push(gauge!(PGMAJFAULT, vmstat.get_pgmajfault()));
				buffer.push(gauge!(PGSCAN_DIRECT_THROTTLE, vmstat.get_pgscan_direct_throttle()));
				buffer.push(gauge!(PGINODESTEAL, vmstat.get_pginodesteal()));
				buffer.push(gauge!(SLABS_SCANNED, vmstat.get_slabs_scanned()));
				buffer.push(gauge!(KSWAPD_INODESTEAL, vmstat.get_kswapd_inodesteal()));
				buffer.push(gauge!(
					KSWAPD_LOW_WMARK_HIT_QUICKLY,
					vmstat.get_kswapd_low_wmark_hit_quickly()
				));
				buffer.push(gauge!(
					KSWAPD_HIGH_WMARK_HIT_QUICKLY,
					vmstat.get_kswapd_high_wmark_hit_quickly()
				));
				buffer.push(gauge!(PAGEOUTRUN, vmstat.get_pageoutrun()));
				buffer.push(gauge!(PGROTATED, vmstat.get_pgrotated()));
				buffer.push(gauge!(DROP_PAGECACHE, vmstat.get_drop_pagecache()));
				buffer.push(gauge!(DROP_SLAB, vmstat.get_drop_slab()));
				buffer.push(gauge!(NUMA_PTE_UPDATES, vmstat.get_numa_pte_updates()));
				buffer.push(gauge!(NUMA_HUGE_PTE_UPDATES, vmstat.get_numa_huge_pte_updates()));
				buffer.push(gauge!(NUMA_HINT_FAULTS, vmstat.get_numa_hint_faults()));
				buffer.push(gauge!(NUMA_HINT_FAULTS_LOCAL, vmstat.get_numa_hint_faults_local()));
				buffer.push(gauge!(NUMA_PAGES_MIGRATED, vmstat.get_numa_pages_migrated()));
				buffer.push(gauge!(PGMIGRATE_SUCCESS, vmstat.get_pgmigrate_success()));
				buffer.push(gauge!(PGMIGRATE_FAIL, vmstat.get_pgmigrate_fail()));
				buffer.push(gauge!(COMPACT_MIGRATE_SCANNED, vmstat.get_compact_migrate_scanned()));
				buffer.push(gauge!(COMPACT_FREE_SCANNED, vmstat.get_compact_free_scanned()));
				buffer.push(gauge!(COMPACT_ISOLATED, vmstat.get_compact_isolated()));
				buffer.push(gauge!(COMPACT_STALL, vmstat.get_compact_stall()));
				buffer.push(gauge!(COMPACT_FAIL, vmstat.get_compact_fail()));
				buffer.push(gauge!(COMPACT_SUCCESS, vmstat.get_compact_success()));
				buffer
					.push(gauge!(HTLB_BUDDY_ALLOC_SUCCESS, vmstat.get_htlb_buddy_alloc_success()));
				buffer.push(gauge!(HTLB_BUDDY_ALLOC_FAIL, vmstat.get_htlb_buddy_alloc_fail()));
				buffer.push(gauge!(UNEVICTABLE_PGS_CULLED, vmstat.get_unevictable_pgs_culled()));
				buffer.push(gauge!(UNEVICTABLE_PGS_SCANNED, vmstat.get_unevictable_pgs_scanned()));
				buffer.push(gauge!(UNEVICTABLE_PGS_RESCUED, vmstat.get_unevictable_pgs_rescued()));
				buffer.push(gauge!(UNEVICTABLE_PGS_MLOCKED, vmstat.get_unevictable_pgs_mlocked()));
				buffer.push(gauge!(
					UNEVICTABLE_PGS_MUNLOCKED,
					vmstat.get_unevictable_pgs_munlocked()
				));
				buffer.push(gauge!(UNEVICTABLE_PGS_CLEARED, vmstat.get_unevictable_pgs_cleared()));
				buffer
					.push(gauge!(UNEVICTABLE_PGS_STRANDED, vmstat.get_unevictable_pgs_stranded()));
				buffer.push(gauge!(THP_FAULT_ALLOC, vmstat.get_thp_fault_alloc()));
				buffer.push(gauge!(THP_FAULT_FALLBACK, vmstat.get_thp_fault_fallback()));
				buffer.push(gauge!(THP_COLLAPSE_ALLOC, vmstat.get_thp_collapse_alloc()));
				buffer.push(gauge!(
					THP_COLLAPSE_ALLOC_FAILED,
					vmstat.get_thp_collapse_alloc_failed()
				));
				buffer.push(gauge!(THP_ZERO_PAGE_ALLOC, vmstat.get_thp_zero_page_alloc()));
				buffer.push(gauge!(
					THP_ZERO_PAGE_ALLOC_FAILED,
					vmstat.get_thp_zero_page_alloc_failed()
				));
				buffer.push(gauge!(BALLOON_INFLATE, vmstat.get_balloon_inflate()));
				buffer.push(gauge!(BALLOON_DEFLATE, vmstat.get_balloon_deflate()));
				buffer.push(gauge!(BALLOON_MIGRATE, vmstat.get_balloon_migrate()));
			},
			Err(error) => {
				warn!("Failed to collect vmstat metrics: {error}");
				return Err(io::Error::last_os_error().into());
			},
		}
		Ok(())
	}
}
