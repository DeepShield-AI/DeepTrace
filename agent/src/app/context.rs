use super::AgentError;
use crate::config::AppConfig;
use arc_swap::ArcSwap;
use log::{error, info};
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use tokio::{
	runtime::{Builder, Runtime},
	sync::OnceCell,
};

#[derive(Debug)]
pub(crate) struct Context {
	pub config: Arc<ArcSwap<AppConfig>>,
	pub runtime: Arc<Runtime>,
	pub state: Arc<AtomicBool>,
}

pub(crate) static CONTEXT: OnceCell<Context> = OnceCell::const_new();

impl Context {
	pub fn new(config: AppConfig, runtime: Runtime) -> Self {
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
		.on_thread_start(|| {
			// 	unshare(libc::CLONE_FS).expect("cannot initialize thread with unshare(CLONE_FS)")
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

pub fn state() -> Arc<AtomicBool> {
	context().state.clone()
}

pub fn terminate() {
	if !context().state.swap(true, Ordering::Relaxed) {
		info!("Agent state changed to terminate");
	}
}
