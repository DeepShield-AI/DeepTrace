use super::AgentError;
use crate::config::AppConfig;
use arc_swap::ArcSwap;
use libc::unshare;
use log::error;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::{
	runtime::{Builder, Runtime},
	sync::OnceCell,
};

#[derive(Debug)]
pub struct Context {
	pub(crate) config: Arc<ArcSwap<AppConfig>>,
	pub(crate) runtime: Arc<Runtime>,
	pub(crate) state: Arc<AtomicBool>,
}

pub(crate) static CONTEXT: OnceCell<Context> = OnceCell::const_new();

impl Context {
	pub(crate) fn new(config: AppConfig, runtime: Runtime) -> Self {
		let config = Arc::new(ArcSwap::from_pointee(config));
		let state = Arc::new(AtomicBool::new(false));
		Self { config, runtime: Arc::new(runtime), state }
	}
}

pub fn init(config_path: impl AsRef<str>) -> Result<(), AgentError> {
	let config = AppConfig::load(config_path)?;
	let runtime = Builder::new_multi_thread()
		.thread_name("deeptrace-worker")
		.worker_threads(config.agent.workers)
		.on_thread_start(|| unsafe {
			unshare(libc::CLONE_FS);
		})
		.enable_all()
		.build()?;
	let context = Context::new(config, runtime);
	CONTEXT.set(context).inspect_err(|_| error!("Failed to set runtime"))?;

	Ok(())
}

pub(crate) fn context() -> &'static Context {
	CONTEXT.get().expect("Runtime not initialized")
}
