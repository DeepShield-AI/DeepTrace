//! Kernel compatibility checking for eBPF programs
//!
//! This module provides utilities to check kernel version requirements
//! for different eBPF program types and hook points.

use aya::util::KernelVersion;
use std::sync::OnceLock;

#[macro_export]
macro_rules! kernel {
	($major:literal) => {
		aya::util::KernelVersion::new($major, 0, 0)
	};
	($major:literal, $minor:literal) => {
		aya::util::KernelVersion::new($major, $minor, 0)
	};
	($major:literal, $minor:literal, $patch:literal) => {
		aya::util::KernelVersion::new($major, $minor, $patch)
	};
}

pub(crate) fn max_version<'a>() -> &'a KernelVersion {
	static MAX_VERSION: OnceLock<KernelVersion> = OnceLock::new();
	MAX_VERSION.get_or_init(|| KernelVersion::new(6, 17, 1))
}

pub(crate) fn min_version<'a>() -> &'a KernelVersion {
	static MIN_VERSION: OnceLock<KernelVersion> = OnceLock::new();
	MIN_VERSION.get_or_init(|| KernelVersion::new(4, 1, 0))
}

// /// Check if a tracepoint exists in the system
// pub fn check_tracepoint(category: &str, name: &str) -> Result<bool> {
// 	#[cfg(target_os = "linux")]
// 	{
// 		// Try tracefs first
// 		let tracefs_path = format!("/sys/kernel/tracing/events/{}/{}/id", category, name);
// 		if std::path::Path::new(&tracefs_path).exists() {
// 			debug!("Found tracepoint: {}/{}", category, name);
// 			return Ok(true);
// 		}

// 		// Fallback to debugfs
// 		let debugfs_path = format!("/sys/kernel/debug/tracing/events/{}/{}/id", category, name);
// 		if std::path::Path::new(&debugfs_path).exists() {
// 			debug!("Found tracepoint in debugfs: {}/{}", category, name);
// 			return Ok(true);
// 		}

// 		debug!("Tracepoint not found: {}/{}", category, name);
// 		Ok(false)
// 	}

// 	#[cfg(not(target_os = "linux"))]
// 	Ok(false)
// }

// /// Check if a kprobe symbol exists
// pub fn check_kprobe_symbol(symbol: &str) -> Result<bool> {
// 	#[cfg(target_os = "linux")]
// 	{
// 		let file = fs::File::open("/proc/kallsyms")?;
// 		let reader = io::BufReader::new(file);

// 		for line in reader.lines() {
// 			let line = line?;
// 			let parts: Vec<&str> = line.split_whitespace().collect();
// 			// kallsyms format: address type symbol [module]
// 			if parts.len() >= 3 && parts[2] == symbol {
// 				debug!("Found kprobe symbol: {}", symbol);
// 				return Ok(true);
// 			}
// 		}

// 		debug!("Kprobe symbol not found: {}", symbol);
// 		Ok(false)
// 	}

// 	#[cfg(not(target_os = "linux"))]
// 	Ok(false)
// }

// /// Check if the process has necessary BPF capabilities
// pub fn check_capabilities() -> Result<()> {
// 	#[cfg(target_os = "linux")]
// 	{
// 		let kernel = KernelVersion::current()?;

// 		// CAP_BPF = 39, CAP_SYS_ADMIN = 21 (from linux/capability.h)
// 		const CAP_BPF: i32 = 39;
// 		const CAP_SYS_ADMIN: i32 = 21;

// 		// For kernels >= 5.8, CAP_BPF is preferred but CAP_SYS_ADMIN also works
// 		if kernel >= KernelVersion::new(5, 8, 0) {
// 			let has_cap_bpf = check_capability(CAP_BPF);
// 			let has_cap_sys_admin = check_capability(CAP_SYS_ADMIN);

// 			if !has_cap_bpf && !has_cap_sys_admin {
// 				return Err(EbpfError::MissingCapability(
// 					"Need CAP_BPF or CAP_SYS_ADMIN".to_string(),
// 				));
// 			}

// 			if has_cap_bpf {
// 				info!("Has CAP_BPF capability");
// 			} else {
// 				info!("Has CAP_SYS_ADMIN capability");
// 			}
// 		} else {
// 			// For older kernels, CAP_SYS_ADMIN is required
// 			if !check_capability(CAP_SYS_ADMIN) {
// 				return Err(EbpfError::MissingCapability(
// 					"Need CAP_SYS_ADMIN for kernel < 5.8".to_string(),
// 				));
// 			}
// 			info!("Has CAP_SYS_ADMIN capability");
// 		}

// 		Ok(())
// 	}

// 	#[cfg(not(target_os = "linux"))]
// 	Err(EbpfError::MissingCapability("Not running on Linux".to_string()))
// }

// #[cfg(target_os = "linux")]
// fn check_capability(cap: libc::c_int) -> bool {
// 	unsafe {
// 		let ret = libc::prctl(libc::PR_CAPBSET_READ, cap, 0, 0, 0);
// 		ret == 1
// 	}
// }
