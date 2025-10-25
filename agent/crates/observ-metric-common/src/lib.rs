use log::info;
use std::{
	borrow::Cow,
	path::{Path, PathBuf},
	sync::{Once, OnceLock},
};

pub fn init_roots() {
	static INIT: Once = Once::new();

	INIT.call_once(|| {
		match std::env::var_os("PROCFS_ROOT") {
			Some(procfs_root) => {
				info!("PROCFS_ROOT is set in envvars. Using custom: {procfs_root:?} for procfs.");
				set_procfs_root(std::path::PathBuf::from(&procfs_root));
			},
			None => info!("PROCFS_ROOT is unset. Using default '/proc' for procfs root."),
		};

		match std::env::var_os("SYSFS_ROOT") {
			Some(sysfs_root) => {
				info!("SYSFS_ROOT is set in envvars. Using custom: {sysfs_root:?} for sysfs.");
				set_sysfs_root(std::path::PathBuf::from(&sysfs_root));
			},
			None => info!("SYSFS_ROOT is unset. Using default '/sys' for sysfs root."),
		}
	});
}

/// Holds global state for custom paths. Static's leveraging this type can only be set ONCE per binary run.
static PROCFS_ROOT: OnceLock<PathBuf> = OnceLock::new();
static SYSFS_ROOT: OnceLock<PathBuf> = OnceLock::new();

/// Instantiates the OnceLock holding PROCFS_ROOT in the case that it hasn't already been instantiated.
///
/// This can only be set ONCE per binary run.
pub fn set_procfs_root<T: Into<Cow<'static, Path>>>(root: T) {
	let root = root.into().into_owned();
	let _ = PROCFS_ROOT.get_or_init(|| root);
}

/// Returns the static `Path` value of a configured PROCFS_ROOT.
///
/// If uninitialized, initializes the PROCFS_ROOT with the default path - `/proc`
pub fn procfs_root() -> &'static Path {
	PROCFS_ROOT.get_or_init(|| PathBuf::from("/proc")).as_ref()
}

/// Instantiates the OnceLock holding SYSFS_ROOT in the case that it hasn't already been instantiated.
///
/// This can only be set ONCE per binary run.
pub fn set_sysfs_root<T: Into<Cow<'static, Path>>>(root: T) {
	let root = root.into().into_owned();
	let _ = SYSFS_ROOT.get_or_init(|| root);
}

/// Returns the static `Path` value of a configured PROCFS_ROOT.
///
/// If uninitialized, initializes the PROCFS_ROOT with the default path - `/sys`
pub fn sysfs_root() -> &'static Path {
	SYSFS_ROOT.get_or_init(|| PathBuf::from("/sys")).as_ref()
}
