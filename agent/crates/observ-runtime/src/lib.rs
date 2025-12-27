//! Observ tokio runtime.
//!
//! This crate provides runtime utilities for the observ system:
//! - Global multi-threaded tokio runtime with configurable thread count
//! - [`NoStealRuntime`]: Multi-threaded runtime without work-stealing for latency-sensitive workloads
//! - [`metrics::Monitor`]: Optional tokio-metrics integration for runtime monitoring

use log::warn;
use std::{io, sync::Arc};
use tokio::{
	runtime::{Builder, Runtime},
	sync::OnceCell,
	task::JoinHandle,
};

static RUNTIME: OnceCell<Arc<Runtime>> = OnceCell::const_new();
pub fn init_runtime() {
	let threads = std::env::var("OBSERV_RT_THREADS")
		.ok()
		.and_then(|s| s.parse().ok())
		.unwrap_or(num_cpus::get().max(16));
	let mut builder = Builder::new_multi_thread();
	let mut builder = builder
		.thread_name_fn(move || format!("observ-pool"))
		.worker_threads(threads)
		.enable_all();
	if cfg!(target_os = "linux") && cfg!(feature = "ebpf") {
		// TODO: why unshare(CLONE_FS) here?
		builder = builder.on_thread_start(|| unsafe {
			let rc = libc::unshare(libc::CLONE_FS);
			if rc != 0 {
				warn!("unshare(CLONE_FS) failed: {}", io::Error::last_os_error());
			}
		});
	}
	let runtime = builder.build().expect("Failed to create observ runtime");
	let _ = RUNTIME.set(Arc::new(runtime)).unwrap();
}
fn runtime() -> &'static Runtime {
	RUNTIME.get().expect("get runtime error")
}

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
	F: Future<Output = T> + Send + 'static,
	T: Send + 'static,
{
	runtime().spawn(f)
}

pub fn spawn_blocking<F, T>(f: F) -> JoinHandle<T>
where
	F: FnOnce() -> T + Send + 'static,
	T: Send + 'static,
{
	runtime().spawn_blocking(f)
}

pub fn block_on<F, T>(f: F) -> T
where
	F: Future<Output = T>,
{
	runtime().block_on(f)
}
