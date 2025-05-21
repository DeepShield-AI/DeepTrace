use crate::AgentError;
pub(crate) use context::{Context, context};
pub(crate) use module::Module;
use tokio::task::JoinHandle;
pub(crate) mod runtime;
pub(crate) use api::config_listener;

mod api;
mod context;
mod lifecycle;
mod module;

pub struct App {
	handle: Option<JoinHandle<Result<(), AgentError>>>,
}
