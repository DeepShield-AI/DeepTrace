use anyhow::Result;
use approx::relative_eq;
use observ_cpu::stat::Stat;
use observ_disk::diskstat::{DISK_SECTOR_SIZE, DiskStat};
use observ_memory::{meminfo::MemInfo, vmstat::VmStat};
use observ_network::netdev::Interface;
use pretty_assertions::assert_eq;
use prism_fixtures::generators::{
	FakeDiskStats, FakeInterfaces, FakeMemInfo, FakeStat, FakeVmStat,
};
use test_utils::convert::clock_ticks;
use uom::si::{
	information::{byte, kilobyte},
	time::{millisecond, second},
};
/// Validator that compares parsed results with original procfs values
pub struct Validator;

impl Validator {
	pub fn new() -> Self {
		Self
	}

	pub fn validate_cpu(&self, fake: FakeStat, real: Stat) -> Result<()> {
		println!("    Validating CPU metrics");

		// Get CPU times for detailed validation
		let real_cpu_times: Vec<_> = real.cpu_times().into_iter().collect();
		assert_eq!(real_cpu_times.len(), fake.cpus.len(), "CPU count mismatch");

		for (fake, (id, real)) in fake.cpus.iter().zip(real_cpu_times.iter()) {
			assert_eq!(
				true,
				relative_eq!(fake.user as f64 / clock_ticks(), real.get_user().get::<second>()),
				"CPU{} user time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.nice as f64 / clock_ticks(), real.get_nice().get::<second>()),
				"CPU{} nice time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.system as f64 / clock_ticks(), real.get_system().get::<second>()),
				"CPU{} system time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.idle as f64 / clock_ticks(), real.get_idle().get::<second>()),
				"CPU{} idle time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.iowait as f64 / clock_ticks(), real.get_iowait().get::<second>()),
				"CPU{} iowait time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.irq as f64 / clock_ticks(), real.get_irq().get::<second>()),
				"CPU{} irq time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.softirq as f64 / clock_ticks(),
					real.get_softirq().get::<second>()
				),
				"CPU{} softirq time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.steal as f64 / clock_ticks(), real.get_steal().get::<second>()),
				"CPU{} steal time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(fake.guest as f64 / clock_ticks(), real.get_guest().get::<second>()),
				"CPU{} guest time mismatch",
				id
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.guest_nice as f64 / clock_ticks(),
					real.get_guest_nice().get::<second>()
				),
				"CPU{} guest nice time mismatch",
				id
			);
		}
		let real_cpu_total = real.cpu_total();
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.user as f64 / clock_ticks(),
				real_cpu_total.get_user().get::<second>()
			),
			"Total CPU user time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.nice as f64 / clock_ticks(),
				real_cpu_total.get_nice().get::<second>()
			),
			"Total CPU nice time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.system as f64 / clock_ticks(),
				real_cpu_total.get_system().get::<second>()
			),
			"Total CPU system time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.idle as f64 / clock_ticks(),
				real_cpu_total.get_idle().get::<second>()
			),
			"Total CPU idle time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.iowait as f64 / clock_ticks(),
				real_cpu_total.get_iowait().get::<second>()
			),
			"Total CPU iowait time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.irq as f64 / clock_ticks(),
				real_cpu_total.get_irq().get::<second>()
			),
			"Total CPU irq time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.softirq as f64 / clock_ticks(),
				real_cpu_total.get_softirq().get::<second>()
			),
			"Total CPU softirq time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.steal as f64 / clock_ticks(),
				real_cpu_total.get_steal().get::<second>()
			),
			"Total CPU steal time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.guest as f64 / clock_ticks(),
				real_cpu_total.get_guest().get::<second>()
			),
			"Total CPU guest time mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.cpu_total.guest_nice as f64 / clock_ticks(),
				real_cpu_total.get_guest_nice().get::<second>()
			),
			"Total CPU guest nice time mismatch"
		);

		assert_eq!(fake.context_switches, real.context_switches(), "Context switches mismatch");
		assert_eq!(fake.boot_time, real.boot_time(), "Boot time mismatch");
		assert_eq!(fake.processes, real.processes(), "Processes mismatch");
		assert_eq!(fake.procs_running, real.procs_running(), "Running processes mismatch");
		assert_eq!(fake.procs_blocked, real.procs_blocked(), "Blocked processes mismatch");
		println!("      CPU field validation successful");
		Ok(())
	}

	pub fn validate_meminfo(&self, fake: FakeMemInfo, real: MemInfo) -> Result<()> {
		println!("    Validating Memory metrics");

		assert_eq!(
			true,
			relative_eq!(
				fake.MemTotal as f64,
				real.get_MemTotal().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory total mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.MemFree as f64,
				real.get_MemFree().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory free mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.MemAvailable as f64,
				real.get_MemAvailable().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory available mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Buffers as f64,
				real.get_Buffers().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory buffers mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Cached as f64,
				real.get_Cached().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory cached mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.SwapCached as f64,
				real.get_SwapCached().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory swap cached mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Active as f64,
				real.get_Active().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory active mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Inactive as f64,
				real.get_Inactive().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Active_anon as f64,
				real.get_Active_anon().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Inactive_anon as f64,
				real.get_Inactive_anon().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Active_file as f64,
				real.get_Active_file().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Inactive_file as f64,
				real.get_Inactive_file().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.SwapTotal as f64,
				real.get_SwapTotal().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory swap total mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.SwapFree as f64,
				real.get_SwapFree().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Dirty as f64,
				real.get_Dirty().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory dirty mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Writeback as f64,
				real.get_Writeback().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory writeback mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.AnonPages as f64,
				real.get_AnonPages().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory anon pages mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Mapped as f64,
				real.get_Mapped().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory mapped mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Shmem as f64,
				real.get_Shmem().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory shmem mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.KReclaimable as f64,
				real.get_KReclaimable().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory k reclaimable mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Slab as f64,
				real.get_Slab().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory slab mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.SReclaimable as f64,
				real.get_SReclaimable().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory sreclaimable mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.SUnreclaim as f64,
				real.get_SUnreclaim().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory sunreclaim mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.KernelStack as f64,
				real.get_KernelStack().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory kernel stack mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.PageTables as f64,
				real.get_PageTables().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.NFS_Unstable as f64,
				real.get_NFS_Unstable().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory nfs unstable mismatch {}, {}",
			fake.NFS_Unstable,
			real.get_NFS_Unstable().get::<kilobyte>()
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Bounce as f64,
				real.get_Bounce().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory bounce mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.WritebackTmp as f64,
				real.get_WritebackTmp().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory writeback tmp mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.CommitLimit as f64,
				real.get_CommitLimit().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory commit limit mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.Committed_AS as f64,
				real.get_Committed_AS().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory committed AS mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.VmallocTotal as f64,
				real.get_VmallocTotal().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory vmalloc total mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.VmallocUsed as f64,
				real.get_VmallocUsed().get::<kilobyte>(),
				epsilon = f64::EPSILON,
			),
			"Memory vmalloc used mismatch"
		);
		assert_eq!(
			true,
			relative_eq!(
				fake.VmallocChunk as f64,
				real.get_VmallocChunk().get::<kilobyte>() as f64,
				epsilon = f64::EPSILON,
			),
			"Memory vmalloc chunk mismatch"
		);
		println!("      Memory field validation successful");
		Ok(())
	}

	pub fn validate_vmstat(&self, fake: FakeVmStat, real: VmStat) -> Result<()> {
		println!("    Validating VmStat metrics");

		assert_eq!(fake.nr_free_pages, *real.get_nr_free_pages(), "VmStat nr_free_pages mismatch");
		assert_eq!(
			fake.nr_inactive_anon,
			*real.get_nr_inactive_anon(),
			"VmStat nr_inactive_anon mismatch"
		);
		assert_eq!(
			fake.nr_active_anon,
			*real.get_nr_active_anon(),
			"VmStat nr_active_anon mismatch"
		);
		assert_eq!(
			fake.nr_inactive_file,
			*real.get_nr_inactive_file(),
			"VmStat nr_inactive_file mismatch"
		);
		assert_eq!(
			fake.nr_active_file,
			*real.get_nr_active_file(),
			"VmStat nr_active_file mismatch"
		);
		assert_eq!(
			fake.nr_unevictable,
			*real.get_nr_unevictable(),
			"VmStat nr_unevictable mismatch"
		);
		assert_eq!(fake.nr_mlock, *real.get_nr_mlock(), "VmStat nr_mlock mismatch");
		assert_eq!(fake.nr_anon_pages, *real.get_nr_anon_pages(), "VmStat nr_anon_pages mismatch");
		assert_eq!(fake.nr_mapped, *real.get_nr_mapped(), "VmStat nr_mapped mismatch");
		assert_eq!(fake.nr_file_pages, *real.get_nr_file_pages(), "VmStat nr_file_pages mismatch");
		assert_eq!(fake.nr_dirty, *real.get_nr_dirty(), "VmStat nr_dirty mismatch");
		assert_eq!(fake.nr_writeback, *real.get_nr_writeback(), "VmStat nr_writeback mismatch");
		assert_eq!(
			fake.nr_slab_reclaimable,
			*real.get_nr_slab_reclaimable(),
			"VmStat nr_slab_reclaimable mismatch"
		);
		assert_eq!(
			fake.nr_slab_unreclaimable,
			*real.get_nr_slab_unreclaimable(),
			"VmStat nr_slab_unreclaimable mismatch"
		);
		assert_eq!(
			fake.nr_page_table_pages,
			*real.get_nr_page_table_pages(),
			"VmStat nr_page_table_pages mismatch"
		);
		assert_eq!(
			fake.nr_kernel_stack,
			*real.get_nr_kernel_stack(),
			"VmStat nr_kernel_stack mismatch"
		);
		assert_eq!(fake.nr_unstable, *real.get_nr_unstable(), "VmStat nr_unstable mismatch");
		assert_eq!(fake.nr_bounce, *real.get_nr_bounce(), "VmStat nr_bounce mismatch");
		assert_eq!(
			fake.nr_vmscan_write,
			*real.get_nr_vmscan_write(),
			"VmStat nr_vmscan_write mismatch"
		);
		assert_eq!(
			fake.nr_vmscan_immediate_reclaim,
			*real.get_nr_vmscan_immediate_reclaim(),
			"VmStat nr_vmscan_immediate_reclaim mismatch"
		);
		assert_eq!(
			fake.nr_writeback_temp,
			*real.get_nr_writeback_temp(),
			"VmStat nr_writeback_temp mismatch"
		);
		assert_eq!(
			fake.nr_isolated_anon,
			*real.get_nr_isolated_anon(),
			"VmStat nr_isolated_anon mismatch"
		);
		assert_eq!(
			fake.nr_isolated_file,
			*real.get_nr_isolated_file(),
			"VmStat nr_isolated_file mismatch"
		);
		assert_eq!(fake.nr_shmem, *real.get_nr_shmem(), "VmStat nr_shmem mismatch");
		assert_eq!(fake.nr_dirtied, *real.get_nr_dirtied(), "VmStat nr_dirtied mismatch");
		assert_eq!(fake.nr_written, *real.get_nr_written(), "VmStat nr_written mismatch");
		assert_eq!(fake.numa_hit, *real.get_numa_hit(), "VmStat numa_hit mismatch");
		assert_eq!(fake.numa_miss, *real.get_numa_miss(), "VmStat numa_miss mismatch");
		assert_eq!(fake.numa_foreign, *real.get_numa_foreign(), "VmStat numa_foreign mismatch");
		assert_eq!(
			fake.numa_interleave,
			*real.get_numa_interleave(),
			"VmStat numa_interleave mismatch"
		);
		assert_eq!(fake.numa_local, *real.get_numa_local(), "VmStat numa_local mismatch");
		assert_eq!(fake.numa_other, *real.get_numa_other(), "VmStat numa_other mismatch");
		assert_eq!(
			fake.workingset_nodereclaim,
			*real.get_workingset_nodereclaim(),
			"VmStat workingset_nodereclaim mismatch"
		);
		assert_eq!(
			fake.nr_anon_transparent_hugepages,
			*real.get_nr_anon_transparent_hugepages(),
			"VmStat nr_anon_transparent_hugepages mismatch"
		);
		assert_eq!(fake.nr_free_cma, *real.get_nr_free_cma(), "VmStat nr_free_cma mismatch");
		assert_eq!(
			fake.nr_dirty_threshold,
			*real.get_nr_dirty_threshold(),
			"VmStat nr_dirty_threshold mismatch"
		);
		assert_eq!(
			fake.nr_dirty_background_threshold,
			*real.get_nr_dirty_background_threshold(),
			"VmStat nr_dirty_background_threshold mismatch"
		);
		assert_eq!(fake.pgpgin, *real.get_pgpgin(), "VmStat pgpgin mismatch");
		assert_eq!(fake.pgpgout, *real.get_pgpgout(), "VmStat pgpgout mismatch");
		assert_eq!(fake.pswpin, *real.get_pswpin(), "VmStat pswpin mismatch");
		assert_eq!(fake.pswpout, *real.get_pswpout(), "VmStat pswpout mismatch");
		assert_eq!(fake.pgalloc_dma, *real.get_pgalloc_dma(), "VmStat pgalloc_dma mismatch");
		assert_eq!(fake.pgalloc_dma32, *real.get_pgalloc_dma32(), "VmStat pgalloc_dma32 mismatch");
		assert_eq!(
			fake.pgalloc_normal,
			*real.get_pgalloc_normal(),
			"VmStat pgalloc_normal mismatch"
		);
		assert_eq!(
			fake.pgalloc_movable,
			*real.get_pgalloc_movable(),
			"VmStat pgalloc_movable mismatch"
		);
		assert_eq!(fake.pgfree, *real.get_pgfree(), "VmStat pgfree mismatch");
		assert_eq!(fake.pgactivate, *real.get_pgactivate(), "VmStat pgactivate mismatch");
		assert_eq!(fake.pgdeactivate, *real.get_pgdeactivate(), "VmStat pgdeactivate mismatch");
		assert_eq!(fake.pgfault, *real.get_pgfault(), "VmStat pgfault mismatch");
		assert_eq!(fake.pgmajfault, *real.get_pgmajfault(), "VmStat pgmajfault mismatch");
		assert_eq!(
			fake.pgscan_direct_throttle,
			*real.get_pgscan_direct_throttle(),
			"VmStat pgscan_direct_throttle mismatch"
		);
		assert_eq!(fake.pginodesteal, *real.get_pginodesteal(), "VmStat pginodesteal mismatch");
		assert_eq!(fake.slabs_scanned, *real.get_slabs_scanned(), "VmStat slabs_scanned mismatch");
		assert_eq!(
			fake.kswapd_inodesteal,
			*real.get_kswapd_inodesteal(),
			"VmStat kswapd_inodesteal mismatch"
		);
		assert_eq!(
			fake.kswapd_low_wmark_hit_quickly,
			*real.get_kswapd_low_wmark_hit_quickly(),
			"VmStat kswapd_low_wmark_hit_quickly mismatch"
		);
		assert_eq!(
			fake.kswapd_high_wmark_hit_quickly,
			*real.get_kswapd_high_wmark_hit_quickly(),
			"VmStat kswapd_high_wmark_hit_quickly mismatch"
		);
		assert_eq!(fake.pageoutrun, *real.get_pageoutrun(), "VmStat pageoutrun mismatch");
		assert_eq!(fake.pgrotated, *real.get_pgrotated(), "VmStat pgrotated mismatch");
		assert_eq!(
			fake.drop_pagecache,
			*real.get_drop_pagecache(),
			"VmStat drop_pagecache mismatch"
		);
		assert_eq!(fake.drop_slab, *real.get_drop_slab(), "VmStat drop_slab mismatch");
		assert_eq!(
			fake.numa_pte_updates,
			*real.get_numa_pte_updates(),
			"VmStat numa_pte_updates mismatch"
		);
		assert_eq!(
			fake.numa_huge_pte_updates,
			*real.get_numa_huge_pte_updates(),
			"VmStat numa_huge_pte_updates mismatch"
		);
		assert_eq!(
			fake.numa_hint_faults,
			*real.get_numa_hint_faults(),
			"VmStat numa_hint_faults mismatch"
		);
		assert_eq!(
			fake.numa_hint_faults_local,
			*real.get_numa_hint_faults_local(),
			"VmStat numa_hint_faults_local mismatch"
		);
		assert_eq!(
			fake.numa_pages_migrated,
			*real.get_numa_pages_migrated(),
			"VmStat numa_pages_migrated mismatch"
		);
		assert_eq!(
			fake.pgmigrate_success,
			*real.get_pgmigrate_success(),
			"VmStat pgmigrate_success mismatch"
		);
		assert_eq!(
			fake.pgmigrate_fail,
			*real.get_pgmigrate_fail(),
			"VmStat pgmigrate_fail mismatch"
		);
		assert_eq!(
			fake.compact_migrate_scanned,
			*real.get_compact_migrate_scanned(),
			"VmStat compact_migrate_scanned mismatch"
		);
		assert_eq!(
			fake.compact_free_scanned,
			*real.get_compact_free_scanned(),
			"VmStat compact_free_scanned mismatch"
		);
		assert_eq!(
			fake.compact_isolated,
			*real.get_compact_isolated(),
			"VmStat compact_isolated mismatch"
		);
		assert_eq!(fake.compact_stall, *real.get_compact_stall(), "VmStat compact_stall mismatch");
		assert_eq!(fake.compact_fail, *real.get_compact_fail(), "VmStat compact_fail mismatch");
		assert_eq!(fake.compact_success, *real.get_compact_success(), "VmStat compact mismatch");
		assert_eq!(
			fake.htlb_buddy_alloc_success,
			*real.get_htlb_buddy_alloc_success(),
			"VmStat htlb_buddy_alloc_success mismatch"
		);
		assert_eq!(
			fake.htlb_buddy_alloc_fail,
			*real.get_htlb_buddy_alloc_fail(),
			"VmStat htlb_buddy_alloc_fail mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_culled,
			*real.get_unevictable_pgs_culled(),
			"VmStat unevictable_pgs_culled mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_scanned,
			*real.get_unevictable_pgs_scanned(),
			"VmStat unevictable_pgs_scanned mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_rescued,
			*real.get_unevictable_pgs_rescued(),
			"VmStat unevictable_pgs_rescued mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_mlocked,
			*real.get_unevictable_pgs_mlocked(),
			"VmStat unevictable_pgs_mlocked mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_munlocked,
			*real.get_unevictable_pgs_munlocked(),
			"VmStat unevictable_pgs_munlocked mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_cleared,
			*real.get_unevictable_pgs_cleared(),
			"VmStat unevictable_pgs_cleared mismatch"
		);
		assert_eq!(
			fake.unevictable_pgs_stranded,
			*real.get_unevictable_pgs_stranded(),
			"VmStat unevictable_pgs_stranded mismatch"
		);
		assert_eq!(
			fake.thp_fault_alloc,
			*real.get_thp_fault_alloc(),
			"VmStat thp_fault_alloc mismatch"
		);
		assert_eq!(
			fake.thp_fault_fallback,
			*real.get_thp_fault_fallback(),
			"VmStat thp_fault_fallback mismatch"
		);
		assert_eq!(
			fake.thp_collapse_alloc,
			*real.get_thp_collapse_alloc(),
			"VmStat thp_collapse_alloc mismatch"
		);
		assert_eq!(
			fake.thp_collapse_alloc_failed,
			*real.get_thp_collapse_alloc_failed(),
			"VmStat thp_collapse_alloc_failed mismatch"
		);
		assert_eq!(
			fake.thp_zero_page_alloc,
			*real.get_thp_zero_page_alloc(),
			"VmStat thp_zero_page_alloc mismatch"
		);
		assert_eq!(
			fake.thp_zero_page_alloc_failed,
			*real.get_thp_zero_page_alloc_failed(),
			"VmStat thp_zero_page_alloc_failed mismatch"
		);
		assert_eq!(
			fake.balloon_inflate,
			*real.get_balloon_inflate(),
			"VmStat balloon_inflate mismatch"
		);
		assert_eq!(
			fake.balloon_deflate,
			*real.get_balloon_deflate(),
			"VmStat balloon_deflate mismatch"
		);
		assert_eq!(
			fake.balloon_migrate,
			*real.get_balloon_migrate(),
			"VmStat balloon_migrate mismatch"
		);

		println!("      VmStat field validation successful");
		Ok(())
	}

	pub fn validate_diskstat(&self, fake: FakeDiskStats, real: Vec<DiskStat>) -> Result<()> {
		println!("    Validating Disk metrics");

		// Validate device count
		assert_eq!(fake.0.len(), real.len(), "Device count mismatch between fake and real data");

		// Validate individual disk device fields
		for (fake, real) in fake.0.iter().zip(real.iter()) {
			assert_eq!(fake.name, *real.get_name(), "{} Device name mismatch", fake.name);
			assert_eq!(fake.major, *real.get_major(), "{} Device major mismatch", fake.name);
			assert_eq!(fake.minor, *real.get_minor(), "{} Device minor mismatch", fake.name);
			assert_eq!(
				fake.read_completed,
				*real.get_read_completed(),
				"{} Device read completed mismatch",
				fake.name
			);
			assert_eq!(
				fake.read_merged,
				*real.get_read_merged(),
				"{} Device read merged mismatch",
				fake.name
			);
			assert_eq!(
				fake.sectors_read * DISK_SECTOR_SIZE,
				real.get_sectors_read().get::<byte>() as u64,
				"{} Device sectors read mismatch",
				fake.name
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.read_time as f64,
					real.get_read_time().get::<millisecond>(),
					epsilon = f64::EPSILON
				),
				"{} Device read time mismatch",
				fake.name
			);
			assert_eq!(
				fake.write_completed,
				*real.get_write_completed(),
				"{} Device write completed mismatch",
				fake.name
			);
			assert_eq!(
				fake.write_merged,
				*real.get_write_merged(),
				"{} Device write merged mismatch",
				fake.name
			);
			assert_eq!(
				fake.sectors_written * DISK_SECTOR_SIZE,
				real.get_sectors_written().get::<byte>() as u64,
				"{} Device sectors written mismatch",
				fake.name
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.writing_time as f64,
					real.get_writing_time().get::<millisecond>(),
					epsilon = f64::EPSILON
				),
				"{} Device writing time mismatch",
				fake.name
			);
			assert_eq!(
				fake.ios_in_progress,
				*real.get_ios_in_progress(),
				"{} Device ios in progress mismatch",
				fake.name
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.io_time as f64,
					real.get_io_time().get::<millisecond>(),
					epsilon = f64::EPSILON
				),
				"{} Device io time mismatch",
				fake.name
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.weighted_io_time as f64,
					real.get_weighted_io_time().get::<millisecond>(),
					epsilon = f64::EPSILON
				),
				"{} Device weighted io time mismatch",
				fake.name
			);
			assert_eq!(
				fake.discard_completed,
				real.get_discard_completed().unwrap(),
				"{} Device discard completed mismatch",
				fake.name
			);
			assert_eq!(
				fake.discard_merged,
				real.get_discard_merged().unwrap(),
				"{} Device discard merged mismatch",
				fake.name
			);
			assert_eq!(
				fake.sectors_discarded * DISK_SECTOR_SIZE,
				real.get_sectors_discarded().unwrap().get::<byte>() as u64,
				"{} Device sectors discarded mismatch",
				fake.name
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.discarding_time as f64,
					real.get_discarding_time().unwrap().get::<millisecond>(),
					epsilon = f64::EPSILON
				),
				"{} Device discarding time mismatch",
				fake.name
			);
			assert_eq!(
				fake.flush_completed,
				real.get_flush_completed().unwrap(),
				"{} Device flush requests completed mismatch",
				fake.name
			);
			assert_eq!(
				true,
				relative_eq!(
					fake.flushing_time as f64,
					real.get_flushing_time().unwrap().get::<millisecond>(),
					epsilon = f64::EPSILON
				),
				"{} Device flushing time mismatch",
				fake.name
			);
		}
		println!("      Disk field validation successful");
		Ok(())
	}

	pub fn validate_netdev(&self, fake: FakeInterfaces, real: Vec<Interface>) -> Result<()> {
		println!("    Validating Network metrics");

		// Validate interface count
		assert_eq!(fake.0.len(), real.len(), "Interface count mismatch");

		// Validate specific interface values (all fields)
		for (fake, real) in fake.0.iter().zip(real.iter()) {
			assert_eq!(fake.name, real.name, "Interface name mismatch");
			assert_eq!(
				fake.rx_bytes,
				real.get_rx_bytes().get::<byte>() as u64,
				"{} rx_bytes mismatch {}, {}",
				fake.name,
				fake.rx_bytes,
				real.get_rx_bytes().get::<byte>() as u64
			);
			assert_eq!(
				fake.rx_packets,
				*real.get_rx_packets(),
				"{} rx_packets mismatch",
				fake.name
			);
			assert_eq!(fake.rx_errors, *real.get_rx_errors(), "{} rx_errors mismatch", fake.name);
			assert_eq!(
				fake.rx_dropped,
				*real.get_rx_dropped(),
				"{} rx_dropped mismatch",
				fake.name
			);
			assert_eq!(fake.rx_fifo, *real.get_rx_fifo(), "{} rx_fifo mismatch", fake.name);
			assert_eq!(fake.rx_frame, *real.get_rx_frame(), "{} rx_frame mismatch", fake.name);
			assert_eq!(
				fake.rx_compressed,
				*real.get_rx_compressed(),
				"{} rx_compressed mismatch",
				fake.name
			);
			assert_eq!(
				fake.rx_multicast,
				*real.get_rx_multicast(),
				"{} rx_multicast mismatch",
				fake.name
			);
			assert_eq!(
				fake.tx_bytes,
				real.get_tx_bytes().get::<byte>() as usize,
				"{} tx_bytes mismatch",
				fake.name
			);
			assert_eq!(
				fake.tx_packets,
				*real.get_tx_packets(),
				"{} tx_packets mismatch",
				fake.name
			);
			assert_eq!(fake.tx_errors, *real.get_tx_errors(), "{} tx_errors mismatch", fake.name);
			assert_eq!(
				fake.tx_dropped,
				*real.get_tx_dropped(),
				"{} tx_dropped mismatch",
				fake.name
			);
			assert_eq!(fake.tx_fifo, *real.get_tx_fifo(), "{} tx_fifo mismatch", fake.name);
			assert_eq!(fake.tx_colls, *real.get_tx_colls(), "{} tx_collisions mismatch", fake.name);
			assert_eq!(
				fake.tx_carrier,
				*real.get_tx_carrier(),
				"{} tx_carrier mismatch",
				fake.name
			);
			assert_eq!(
				fake.tx_compressed,
				*real.get_tx_compressed(),
				"{} tx_compressed mismatch",
				fake.name
			);
		}
		println!("      ");
		Ok(())
	}
}

// Save parsed results to file
// let mut parsed_content = format!(
// 	"CPUs: {}\nContext switches: {}\nBoot time: {}\nProcesses: {}\nRunning processes: {}\nBlocked processes: {}\n",
// 	cpu_count,
// 	real.context_switches(),
// 	real.boot_time(),
// 	real.processes(),
// 	real.procs_running(),
// 	real.procs_blocked()
// );

// if let Some(cpu) = &cpu_total {
// 	use uom::si::time::second;
// 	parsed_content.push_str(&format!(
//         "CPU Total Times:\n  User: {:.2} seconds\n  Nice: {:.2} seconds\n  System: {:.2} seconds\n  Idle: {:.2} seconds\n  IOWait: {:.2} seconds\n  IRQ: {:.2} seconds\n  SoftIRQ: {:.2} seconds\n  Steal: {:.2} seconds\n  Guest: {:.2} seconds\n  Guest Nice: {:.2} seconds\n\n",
//         cpu.get_user().get::<second>(),
//         cpu.get_nice().get::<second>(),
//         cpu.get_system().get::<second>(),
//         cpu.get_idle().get::<second>(),
//         cpu.get_iowait().get::<second>(),
//         cpu.get_irq().get::<second>(),
//         cpu.get_softirq().get::<second>(),
//         cpu.get_steal().get::<second>(),
//         cpu.get_guest().get::<second>(),
//         cpu.get_guest_nice().get::<second>()
//     ));
// }

// // Add detailed per-CPU times
// parsed_content.push_str("Individual CPU Times:\n");
// for (cpu_id, cpu_time) in &cpu_times {
// 	use uom::si::time::second;
// 	parsed_content.push_str(&format!(
// 		"CPU{}: User={:.2}s, Nice={:.2}s, System={:.2}s, Idle={:.2}s, IOWait={:.2}s, IRQ={:.2}s, SoftIRQ={:.2}s, Steal={:.2}s, Guest={:.2}s, GuestNice={:.2}s\n",
// 		cpu_id,
// 		cpu_time.get_user().get::<second>(),
// 		cpu_time.get_nice().get::<second>(),
// 		cpu_time.get_system().get::<second>(),
// 		cpu_time.get_idle().get::<second>(),
// 		cpu_time.get_iowait().get::<second>(),
// 		cpu_time.get_irq().get::<second>(),
// 		cpu_time.get_softirq().get::<second>(),
// 		cpu_time.get_steal().get::<second>(),
// 		cpu_time.get_guest().get::<second>(),
// 		cpu_time.get_guest_nice().get::<second>()
// 	));
// }
// parsed_content.push('\n');

// fs::write(self.test_dir.join("stat.parsed"), parsed_content)?;

// 		// Save detailed parsed results to file
// 		let parsed_content = format!(
// 			"=== Memory Information ===
// Total Memory: {} kB ({:.2} GB)
// Free Memory: {} kB ({:.2} GB)
// Available Memory: {} kB ({:.2} GB)
// Used Memory: {} kB ({:.2} GB, {:.1}%)

// === Cache and Buffers ===
// Buffers: {} kB ({:.2} MB)
// Cached: {} kB ({:.2} MB)

// === Active/Inactive Memory ===
// Active: {} kB ({:.2} MB)
// Inactive: {} kB ({:.2} MB)
// Active (anon): {} kB ({:.2} MB)
// Inactive (anon): {} kB ({:.2} MB)
// Active (file): {} kB ({:.2} MB)
// Inactive (file): {} kB ({:.2} MB)

// === Swap Information ===
// Swap Total: {} kB ({:.2} GB)
// Swap Free: {} kB ({:.2} GB)
// Swap Used: {} kB ({:.2} GB, {:.1}%)
// ",
// 			mem_total_kb,
// 			mem_total_kb as f64 / 1024.0 / 1024.0,
// 			mem_free_kb,
// 			mem_free_kb as f64 / 1024.0 / 1024.0,
// 			mem_available_kb,
// 			mem_available_kb as f64 / 1024.0 / 1024.0,
// 			mem_used_kb,
// 			mem_used_kb as f64 / 1024.0 / 1024.0,
// 			mem_used_percent,
// 			buffers_kb,
// 			buffers_kb as f64 / 1024.0,
// 			cached_kb,
// 			cached_kb as f64 / 1024.0,
// 			active_kb,
// 			active_kb as f64 / 1024.0,
// 			inactive_kb,
// 			inactive_kb as f64 / 1024.0,
// 			active_anon_kb,
// 			active_anon_kb as f64 / 1024.0,
// 			inactive_anon_kb,
// 			inactive_anon_kb as f64 / 1024.0,
// 			active_file_kb,
// 			active_file_kb as f64 / 1024.0,
// 			inactive_file_kb,
// 			inactive_file_kb as f64 / 1024.0,
// 			swap_total_kb,
// 			swap_total_kb as f64 / 1024.0 / 1024.0,
// 			swap_free_kb,
// 			swap_free_kb as f64 / 1024.0 / 1024.0,
// 			swap_used_kb,
// 			swap_used_kb as f64 / 1024.0 / 1024.0,
// 			swap_used_percent
// 		);
// 		fs::write(self.test_dir.join("meminfo.parsed"), parsed_content)?;

// 		let parsed_content = format!(
// 			"=== Virtual Memory Statistics ===

// === Memory Pages (in pages and KB) ===
// Free Pages: {} pages ({:.2} MB)
// Inactive Anonymous: {} pages ({:.2} MB)
// Active Anonymous: {} pages ({:.2} MB)
// Inactive File: {} pages ({:.2} MB)
// Active File: {} pages ({:.2} MB)
// Anonymous Pages: {} pages ({:.2} MB)
// Mapped Pages: {} pages ({:.2} MB)
// File Pages: {} pages ({:.2} MB)
// Shared Memory: {} pages ({:.2} MB)

// === Dirty/Writeback Pages ===
// Dirty Pages: {} pages ({:.2} MB)
// Writeback Pages: {} pages ({:.2} MB)

// === Slab Memory ===
// Slab Reclaimable: {} pages ({:.2} MB)
// Slab Unreclaimable: {} pages ({:.2} MB)

// === Page I/O Statistics ===
// Pages Read In: {} pages ({:.2} MB total)
// Pages Written Out: {} pages ({:.2} MB total)
// Swap Pages In: {} pages ({:.2} MB total)
// Swap Pages Out: {} pages ({:.2} MB total)

// === Page Fault Statistics ===
// Total Page Faults: {} faults
// Major Page Faults: {} faults
// Minor Page Faults: {} faults

// === NUMA Statistics ===
// NUMA Hit: {} pages
// NUMA Miss: {} pages
// NUMA Hit Rate: {:.2}%
// ",
// 			vmstat.get_nr_free_pages(),
// 			*vmstat.get_nr_free_pages() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_inactive_anon(),
// 			*vmstat.get_nr_inactive_anon() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_active_anon(),
// 			*vmstat.get_nr_active_anon() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_inactive_file(),
// 			*vmstat.get_nr_inactive_file() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_active_file(),
// 			*vmstat.get_nr_active_file() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_anon_pages(),
// 			*vmstat.get_nr_anon_pages() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_mapped(),
// 			*vmstat.get_nr_mapped() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_file_pages(),
// 			*vmstat.get_nr_file_pages() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_shmem(),
// 			*vmstat.get_nr_shmem() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_dirty(),
// 			*vmstat.get_nr_dirty() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_writeback(),
// 			*vmstat.get_nr_writeback() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_slab_reclaimable(),
// 			*vmstat.get_nr_slab_reclaimable() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_nr_slab_unreclaimable(),
// 			*vmstat.get_nr_slab_unreclaimable() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_pgpgin(),
// 			*vmstat.get_pgpgin() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_pgpgout(),
// 			*vmstat.get_pgpgout() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_pswpin(),
// 			*vmstat.get_pswpin() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_pswpout(),
// 			*vmstat.get_pswpout() as f64 * page_size_kb as f64 / 1024.0,
// 			vmstat.get_pgfault(),
// 			vmstat.get_pgmajfault(),
// 			*vmstat.get_pgfault() - *vmstat.get_pgmajfault(),
// 			vmstat.get_numa_hit(),
// 			vmstat.get_numa_miss(),
// 			if *vmstat.get_numa_hit() + *vmstat.get_numa_miss() > 0 {
// 				*vmstat.get_numa_hit() as f64 /
// 					(*vmstat.get_numa_hit() + *vmstat.get_numa_miss()) as f64 *
// 					100.0
// 			} else {
// 				0.0
// 			}
// 		);

// 		fs::write(self.test_dir.join("vmstat.parsed"), parsed_content)?;

// // Save parsed results to file with detailed disk information
// 		let mut parsed_content = "Device count: {}\n", diskstats.len();
// 		for (i, diskstat) in diskstats.iter().enumerate() {
// 			use uom::si::{information::byte, time::millisecond};
// 			parsed_content.push_str(&format!(
//                 "Device {}: major={}, minor={}, name={}, read_completed={}, sectors_read={} bytes, read_time={:.2} ms, write_completed={}, sectors_written={} bytes, write_time={:.2} ms, ios_in_progress={}\n",
//                 i,
//                 diskstat.get_major(),
//                 diskstat.get_minor(),
//                 diskstat.get_name(),
//                 diskstat.get_read_completed(),
//                 diskstat.get_sectors_read().get::<byte>() as u64,
//                 diskstat.get_read_time().get::<millisecond>(),
//                 diskstat.get_write_completed(),
//                 diskstat.get_sectors_written().get::<byte>() as u64,
//                 diskstat.get_writing_time().get::<millisecond>(),
//                 diskstat.get_ios_in_progress()
//             ));
// 		}
// 		fs::write(self.test_dir.join("diskstats.parsed"), parsed_content)?;

// // Save parsed results to file
// let mut parsed_content = "Interface count: {}\n", interfaces.len();
// for interface in &interfaces {
// 	use uom::si::information::byte;
// 	let rx_bytes = interface.rx_bytes.get::<byte>() as u64;
// 	let tx_bytes = interface.tx_bytes.get::<byte>() as u64;
// 	parsed_content.push_str(&format!(
//         "{}: RX {} bytes, {} packets, {} errors, {} dropped | TX {} bytes, {} packets, {} errors, {} dropped\n",
//         interface.name, rx_bytes, interface.rx_packets, interface.rx_errors, interface.rx_dropped,
//         tx_bytes, interface.tx_packets, interface.tx_errors, interface.tx_dropped
//     ));
// }
// fs::write(self.test_dir.join("netdev.parsed"), parsed_content)?;
