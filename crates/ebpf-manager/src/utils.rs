#[cfg(target_os = "linux")]
use libc::{_SC_PAGESIZE, sysconf};
/// Unlock memory limits for eBPF
///
/// This sets RLIMIT_MEMLOCK to unlimited, which is required for loading
/// eBPF programs and maps on older kernels (< 5.11)
use log::{debug, info};
use std::sync::Once;

#[cfg(target_os = "linux")]
pub fn unlock_memory() {
	static ONCE: Once = Once::new();

	ONCE.call_once(|| {
		// Bump the memlock rlimit. This is needed for older kernels that don't use the
		// new memcg based accounting, see https://lwn.net/Articles/837122/
		let rlim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY };
		let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
		if ret != 0 {
			debug!("remove limit on locked memory failed, ret is: {ret}");
		}
		info!("unlocked memory limits");
	});
}

#[cfg(not(target_os = "linux"))]
pub fn unlock_memory() {
	// No-op on non-Linux platforms
}

#[cfg(target_os = "linux")]
fn page_size() -> usize {
	// Safety: libc
	(unsafe { sysconf(_SC_PAGESIZE) }) as usize
}

#[cfg(not(target_os = "linux"))]
fn page_size() -> usize {
	4096_usize
}

#[inline(always)]
pub fn optimal_page_count(max_event_size: usize, n_events: usize) -> usize {
	// Aya's PerfBuffer expects a page_count being a power of two
	// this is something required by the linux kernel
	((max_event_size * n_events) / page_size()).next_power_of_two()
}
