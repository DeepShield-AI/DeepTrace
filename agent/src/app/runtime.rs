use super::context::context;
use std::{sync::Arc, time::Duration};
use tokio::{runtime::Runtime, task::JoinHandle};
fn runtime() -> Arc<Runtime> {
	context().runtime.clone()
}
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
	F: std::future::Future<Output = T> + Send + 'static,
	T: Send + 'static,
{
	runtime().spawn(f)
}

pub fn spawn_blocking<F, T>(f: F) -> JoinHandle<T>
where
	F: std::ops::FnOnce() -> T + Send + 'static,
	T: Send + 'static,
{
	runtime().spawn_blocking(f)
}

pub fn block_on<F, T>(f: F) -> T
where
	F: std::future::Future<Output = T>,
{
	runtime().block_on(f)
}

pub fn sleep(time: Duration) {
	runtime().block_on(tokio::time::sleep(time));
}
