use super::AppConfig;
use crate::app::context;
use log::info;
use std::sync::Arc;
pub fn update_config(new: AppConfig) {
	info!("Updating config: {:?}", new);
	context().config.store(Arc::new(new));
}
