use log::{debug, error};
use nix::{
	sys::signal::{self, Signal},
	unistd::Pid,
};
use std::process;

pub fn unlock_memory() {
	// Bump the memlock rlimit. This is needed for older kernels that don't use the
	// new memcg based accounting, see https://lwn.net/Articles/837122/
	let rlim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY };
	let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
	if ret != 0 {
		debug!("remove limit on locked memory failed, ret is: {ret}");
	}
}

pub fn exit(code: i32) {
	#[cfg(any(target_os = "linux", target_os = "android"))]
	{
		if let Err(e) = signal::kill(Pid::this(), Signal::SIGTERM) {
			error!("Failed to send SIGTERM: {e}");
			process::exit(code);
		}
	}

	process::exit(code);
}
pub fn wait_on_signal() {
	use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};

	let mut signals = Signals::new(TERM_SIGNALS).unwrap();
	log::info!("The Process exits due to signal {:?}.", signals.forever().next());
	signals.handle().close();
}
