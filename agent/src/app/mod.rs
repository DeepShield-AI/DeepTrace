use crate::AgentError;
pub(crate) use context::{Context, context};
pub(crate) use module::Module;
use tokio::task::JoinHandle;
pub(crate) mod runtime;
pub(crate) use state::{state, terminate};

mod context;
mod lifecycle;
mod module;
mod state;

pub struct App {
	handle: Option<JoinHandle<Result<(), AgentError>>>,
}
