use super::context;
use trace_common::protocols::L7Protocol;
use log::info;
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

pub fn state() -> Arc<AtomicBool> {
	context().state.clone()
}

pub fn terminate() {
	if !context().state.swap(true, Ordering::Relaxed) {
		info!("Agent state changed to terminate");
	}
}
