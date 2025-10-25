//! observ tokio runtime.
//!
//! Tokio runtime comes in two flavors: a single-threaded runtime and a multi-
//! threaded one which provides work stealing.
//! Benchmark shows that, compared to the single-threaded runtime, the multi-
//! threaded one has some overhead due to its more sophisticated work steal
//! scheduling.
//!
//! This crate provides a third flavor: a multi-threaded runtime without work
//! stealing. This flavor is as efficient as the single-threaded runtime while
//! allows the async program to use multiple cores.
use log::warn;
use once_cell::sync::{Lazy, OnceCell};
use rand::Rng;
use std::{
	io,
	sync::Arc,
	thread::{self, JoinHandle},
	time::Duration,
};
use thread_local::ThreadLocal;
use tokio::{
	runtime::{Builder, Handle},
	sync::oneshot::{Sender, channel},
};
static CURRENT_HANDLE: Lazy<ThreadLocal<Pools>> = Lazy::new(ThreadLocal::new);

type Control = (Sender<Duration>, JoinHandle<()>);
// TODO: use std::sync::OnceLock when it's stable
type Pools = Arc<OnceCell<Box<[Handle]>>>;

/// observ async multi-threaded runtime
///
/// Multi-threaded runtime backed by a pool of single threaded tokio runtime
struct Runtime {
	threads: usize,
	name: String,
	// Lazily init the runtimes so that they are created after observ daemonize itself. Otherwise the runtime threads are lost.
	pools: Pools,
	controls: OnceCell<Vec<Control>>,
}

impl Runtime {
	/// Create a new [Runtime]. Panic if `threads` is 0
	fn new(threads: usize, name: &str) -> Self {
		assert!(threads != 0);
		Runtime {
			threads,
			name: name.to_string(),
			pools: Arc::new(OnceCell::new()),
			controls: OnceCell::new(),
		}
	}

	fn init_pools(&self) -> (Box<[Handle]>, Vec<Control>) {
		let mut pools = Vec::with_capacity(self.threads);
		let mut controls = Vec::with_capacity(self.threads);
		for id in 0..self.threads {
			let mut builder = Builder::new_current_thread();
			let mut builder =
				builder.thread_name_fn(move || format!("observ-pool-{id}")).enable_all();
			if cfg!(target_os = "linux") && cfg!(feature = "ebpf") {
				// TODO: why unshare(CLONE_FS) here?
				builder = builder.on_thread_start(|| unsafe {
					let rc = libc::unshare(libc::CLONE_FS);
					if rc != 0 {
						warn!("unshare(CLONE_FS) failed: {}", io::Error::last_os_error());
					}
				});
			}
			let rt = builder.build().expect("Failed to create observ runtime");
			let handler = rt.handle().clone();
			let (tx, rx) = channel::<Duration>();
			let pools_ref = Arc::clone(&self.pools);
			let join = thread::Builder::new()
				.name(self.name.clone())
				.spawn(move || {
					CURRENT_HANDLE.get_or(|| pools_ref);
					if let Ok(timeout) = rt.block_on(rx) {
						rt.shutdown_timeout(timeout);
					} // else Err(_): tx is dropped, just exit
				})
				.unwrap();
			pools.push(handler);
			controls.push((tx, join));
		}

		(pools.into_boxed_slice(), controls)
	}

	/// Return the &[Handle] of a random thread of this runtime
	fn get_handle(&self) -> &Handle {
		let mut rng = rand::rng();

		let index = rng.random_range(0..self.threads);
		self.get_runtime_at(index)
	}

	fn get_pools(&self) -> &[Handle] {
		if let Some(p) = self.pools.get() {
			p
		} else {
			// TODO: use a mutex to avoid creating a lot threads only to drop them
			let (pools, controls) = self.init_pools();
			// there could be another thread racing with this one to init the pools
			match self.pools.try_insert(pools) {
				Ok(p) => {
					// unwrap to make sure that this is the one that init both pools and controls
					self.controls.set(controls).unwrap();
					p
				},
				// another thread already set it, just return it
				Err((p, _)) => p,
			}
		}
	}

	/// Return the &[Handle] of a given thread of this runtime
	fn get_runtime_at(&self, index: usize) -> &Handle {
		let pools = self.get_pools();
		&pools[index]
	}

	/// Call tokio's `shutdown_timeout` of all the runtimes. This function is blocking until
	/// all runtimes exit.
	pub fn shutdown_timeout(mut self, timeout: Duration) {
		if let Some(controls) = self.controls.take() {
			let (txs, joins): (Vec<Sender<_>>, Vec<JoinHandle<()>>) = controls.into_iter().unzip();
			for tx in txs {
				let _ = tx.send(timeout); // Err() when rx is dropped
			}
			for join in joins {
				let _ = join.join(); // ignore thread error
			}
		} // else, the controls and the runtimes are not even init yet, just return;
	}
}

fn runtime() -> &'static Runtime {
	static RUNTIME: OnceCell<Runtime> = OnceCell::new();
	RUNTIME.get_or_init(|| {
		let threads = std::env::var("OBSERV_RT_THREADS")
			.ok()
			.and_then(|s| s.parse().ok())
			.unwrap_or(num_cpus::get().max(16));
		Runtime::new(threads, "observ-runtime")
	})
}

pub fn handle<'a>() -> &'a Handle {
	runtime().get_handle()
}

// TODO: add shutdown function to be called at observ exit

#[test]
fn test_no_steal_runtime() {
	use tokio::time::{Duration, sleep};

	// let rt = Runtime::new(2, "test");
	// let h = rt.get_handle();
	let ret = handle().block_on(async {
		sleep(Duration::from_secs(1)).await;
		let h = handle();
		let join = h.spawn(async {
			sleep(Duration::from_secs(1)).await;
		});
		join.await.unwrap();
		1
	});

	assert_eq!(ret, 1);
}

#[test]
fn test_no_steal_shutdown() {
	use tokio::time::{Duration, sleep};

	let rt = Runtime::new(2, "test");
	let h = rt.get_handle();
	let ret = h.block_on(async {
		sleep(Duration::from_secs(1)).await;
		let h = handle();
		let join = h.spawn(async {
			sleep(Duration::from_secs(1)).await;
		});
		join.await.unwrap();
		1
	});
	assert_eq!(ret, 1);

	rt.shutdown_timeout(Duration::from_secs(1));
}
