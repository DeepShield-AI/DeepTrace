use super::{
	AgentConfig, ApiConfig, AppConfig, SenderConfig, TraceConfig,
	sender::{ElasticConfig, FlatFileConfig},
};
use crate::app::context;
use arc_swap::{ArcSwap, access::Map};
use std::sync::Arc;

type Access<C> = Map<Arc<ArcSwap<AppConfig>>, AppConfig, fn(&AppConfig) -> &C>;

pub type TraceAccess = Access<TraceConfig>;
pub type ApiAccess = Access<ApiConfig>;
pub type AgentAccess = Access<AgentConfig>;
pub type SenderAccess = Access<SenderConfig>;
pub type ElasticAccess = Access<ElasticConfig>;
pub type FlatFileAccess = Access<FlatFileConfig>;

fn config() -> Arc<ArcSwap<AppConfig>> {
	context().config.clone()
}

pub fn api_config() -> ApiAccess {
	Map::new(config(), |config: &AppConfig| -> &ApiConfig { &config.api })
}
pub fn trace_config() -> TraceAccess {
	Map::new(config(), |config: &AppConfig| -> &TraceConfig { &config.ebpf })
}

pub fn agent_config() -> AgentAccess {
	Map::new(config(), |config: &AppConfig| -> &AgentConfig { &config.agent })
}

pub fn sender_config() -> SenderAccess {
	Map::new(config(), |config: &AppConfig| -> &SenderConfig { &config.sender })
}

pub fn elastic_config() -> ElasticAccess {
	Map::new(config(), |config: &AppConfig| -> &ElasticConfig { &config.sender.elastic })
}

pub fn flat_file_config() -> FlatFileAccess {
	Map::new(config(), |config: &AppConfig| -> &FlatFileConfig { &config.sender.flat_file })
}
